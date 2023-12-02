#!/usr/bin/env bun
import * as cheerio from 'cheerio';
import { type AppItem, appPaths, writeAppData } from 'glance-app';
import ky from 'ky';
import minimist from 'minimist';
import * as path from 'path';

const args = minimist(Bun.argv.slice(2));
const resummarize = args.resummarize;
const rewrite = args.rewrite;

const appId = 'hackernews';

interface StoryItem {
  by: string;
  descendants: number;
  dead: boolean;
  id: number;
  kids: number[];
  score: number;
  time: number;
  title: string;
  type: 'story';
  url?: string;
}

interface CommentItem {
  by: string;
  id: number;
  kids: number[];
  parent: number;
  text: string;
  time: number;
  type: 'comment';
}

interface DataFile {
  items: DataItem[];
  previous: Record<string, number>;
}

interface DataItem {
  info: StoryItem;
  updated: string;
  comments: string;
  page: string;
  pageSummary?: string;
  commentSummary?: string;
}

const dataFilePath = path.join(appPaths(appId).appStateDir, 'hackernews.json');
const NUM_PAST_STORIES = 20;
const NUM_TOP_STORIES = 5;

let data: DataFile;
try {
  data = await Bun.file(dataFilePath).json();
} catch (e) {
  data = { items: [], previous: {} };
}

if (!data.previous) {
  data.previous = {};
}

async function runPromptBox(template: string, args: string[], input?: string): Promise<string> {
  let proc = Bun.spawn({
    cmd: ['promptbox', 'run', template, ...args],
    stdin: 'pipe',
  });

  if (input) {
    proc.stdin.write(input);
  }
  proc.stdin.end();

  const output = await new Response(proc.stdout).text();
  return output;
}

async function topStoryIds(): Promise<number[]> {
  const stories = await ky('https://hacker-news.firebaseio.com/v0/topstories.json').json<
    number[]
  >();
  return stories.slice(0, NUM_TOP_STORIES);
}

async function pastStoryIds(): Promise<number[]> {
  const page = await ky(`https://news.ycombinator.com/front`).text();
  const $ = cheerio.load(page);
  let ids = $('.athing')
    .slice(0, NUM_PAST_STORIES)
    .map((_i, el) => +el.attribs.id)
    .toArray();
  return ids;
}

async function summarizePage(title: string, contents: string, cached?: DataItem): Promise<string> {
  if (!contents) {
    return '';
  }

  if (cached?.pageSummary && contents === cached?.page && !resummarize) {
    return cached.pageSummary;
  }

  return runPromptBox('summarize-page', ['--title', title], contents);
}

async function summarizeComments(
  title: string,
  contents: string,
  cached?: DataItem
): Promise<string> {
  if (!contents) {
    return '';
  }

  if (cached?.commentSummary && contents.length === cached?.comments.length && !resummarize) {
    return cached.commentSummary;
  }

  let args = [];
  if (title) {
    args.push('--title', title);
  }

  return runPromptBox('summarize-comments', args, contents);
}

function retryDelay(attemptCount: number) {
  return Math.pow(2, attemptCount) * 1000;
}

async function fetchAndProcessStory(itemId: number, cached?: DataItem): Promise<DataItem | null> {
  const [info, hnText] = await Promise.all([
    ky(`https://hacker-news.firebaseio.com/v0/item/${itemId}.json`, {
      retry: { limit: 2, delay: retryDelay },
    }).then((r) => r.json<StoryItem>()),
    ky(`https://news.ycombinator.com/item?id=${itemId}`, {
      retry: { limit: 2, delay: retryDelay },
    }).then((r) => r.text()),
  ]);

  if (info.type !== 'story' || info.dead) {
    return null;
  }

  console.log(info);

  let pageContents = cached?.page ?? '';
  let pageSummary = cached?.pageSummary ?? '';
  if (!pageContents && info.url) {
    try {
      const res = await ky(info.url);
      pageContents = await res.text();
      if (res.headers.get('content-type')?.includes('text/html')) {
        const $ = cheerio.load(pageContents);
        pageContents = $('body').prop('innerText') ?? '';
      }

      pageSummary = await summarizePage(info.title, pageContents, cached);
      console.log('summary', pageSummary);
    } catch (e) {
      console.error(itemId, e);
    }
  }

  const comments = parseHTMLComments(hnText);

  let commentSummary = await summarizeComments(info.title, comments, cached);
  console.log('comments', commentSummary);

  let result = {
    info,
    comments,
    page: pageContents || '',
    pageSummary,
    commentSummary,
  };

  let updated: string;
  if (cached) {
    let { updated: cachedUpdated, ...cachedCompare } = cached;
    if (Bun.deepEquals(cachedCompare, result)) {
      updated = cachedUpdated;
    } else {
      updated = new Date().toISOString();
    }
  } else {
    updated = new Date().toISOString();
  }

  return {
    ...result,
    updated,
  };
}

function parseHTMLComments(html: string): string {
  const $ = cheerio.load(html);
  return $('.commtext')
    .map((_i, el) => {
      $(el).find('.reply').remove();
      return el;
    })
    .text();
}

async function run(): Promise<AppItem[]> {
  let stories: DataItem[] = [];

  if (rewrite) {
    stories = data.items;
  } else if (resummarize) {
    for (let item of data.items) {
      let pageSummary = await summarizePage(item.info.title, item.page, item);
      let commentSummary = await summarizeComments(item.info.title, item.comments, item);

      if (pageSummary !== item.pageSummary || commentSummary !== item.commentSummary) {
        item.pageSummary = pageSummary;
        item.commentSummary = commentSummary;
        item.updated = new Date().toISOString();
      }
    }

    stories = data.items;
  } else {
    const pastItems = await pastStoryIds();
    const topItems = await topStoryIds();
    const uniqueItems = new Set([...pastItems, ...topItems]);

    const itemCache = Object.fromEntries(data.items.map((item) => [item.info.id, item]));

    for (let itemId of [...uniqueItems]) {
      const existing = itemCache[itemId];
      if (data.previous[itemId] && !existing) {
        // We already processed this item and it dropped off the list, so don't put it back on.
        continue;
      }

      const result = await fetchAndProcessStory(itemId, itemCache[itemId]);
      if (result) {
        stories.push(result);
        data.previous[itemId] = result.info.time;
      }
    }
  }

  // Stop tracking items older than 1 week
  const cutoffTime = Date.now() / 1000 - 60 * 60 * 24 * 7;
  for (let [item, time] of Object.entries(data.previous)) {
    if (time < cutoffTime) {
      delete data.previous[item];
    }
  }

  data.items = stories;
  return stories.map((story) => {
    let date = new Date(story.info.time * 1000);
    const subtitle = `${date.toDateString()}, ${story.info.score} votes, ${
      story.info.descendants
    } comments`;

    let commentsUrl = encodeURI(`https://news.ycombinator.com/item?id=${story.info.id}`);
    let commentsHeader = `From the <a href="${commentsUrl}">comments</a>:`;
    let comments = story.commentSummary ? `${commentsHeader}\n${story.commentSummary.trim()}` : '';

    const detail = [story.pageSummary?.trim(), comments].filter(Boolean).join('\n\n');

    return {
      id: story.info.id.toString(),
      updated: story.updated,
      // We never want to resurface these items after they're dismissed, so always use the same state_key.
      state_key: story.info.id.toString(),
      data: {
        title: story.info.title,
        subtitle,
        url: story.info.url,
        detail: `${detail.replaceAll('\n', '<br />')}`,
      },
    };
  }) satisfies AppItem[];
}

const items = await run();
console.log(items);

Bun.write(dataFilePath, JSON.stringify(data));

writeAppData(appId, {
  name: 'Hacker News',
  path: __filename,
  items,
  schedule: [
    {
      cron: '0 */3 * * *',
    },
  ],
});
