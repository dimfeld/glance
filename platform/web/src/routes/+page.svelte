<script lang="ts">
  import SideLabelled from '$lib/components/SideLabelled.svelte';
  // import * as Card from '$lib/components/ui/card';
  // import Switch from '$lib/components/ui/switch/switch.svelte';
  // import { Button } from '$lib/components/ui/button';
  import { mdiEmailOutline, mdiEmailOpenOutline } from '@mdi/js';
  import { enhance } from '$app/forms';
  import { onDestroy, onMount } from 'svelte';
  import { invalidateAll } from '$app/navigation';
  import { browser } from '$app/environment';
  import { Button, Card, Switch, getSettings } from 'svelte-ux';
  import DarkModeSwitch from '$lib/components/DarkModeSwitch.svelte';

  export let data;

  let showDismissed = false;

  const { currentTheme } = getSettings();

  $: apps = data.apps
    .map((app) => {
      return {
        ...app,
        items: app.items.filter((item) => showDismissed || !item.dismissed),
      };
    })
    .filter((app) => app.items.length > 0);

  let canRefresh = browser ? document.visibilityState === 'visible' : false;
  let lastRefresh = Date.now();
  let refreshTimer: number | null = null;
  const REFRESH_TIME = 15 * 60 * 1000;

  function doRefresh() {
    refreshTimer = null;
    lastRefresh = Date.now();
    invalidateAll();

    if (canRefresh) {
      setRefreshTimer();
    }
  }

  function setRefreshTimer() {
    console.trace('setRefreshTimer');
    const timeSinceRefresh = Date.now() - lastRefresh;
    const time = Math.max(0, REFRESH_TIME - timeSinceRefresh);
    refreshTimer = setTimeout(doRefresh, time);
  }

  $: if (browser) {
    if (canRefresh && !refreshTimer) {
      setRefreshTimer();
    } else if (refreshTimer && !canRefresh) {
      clearTimeout(refreshTimer);
      refreshTimer = null;
    }
  }

  onDestroy(() => {
    if (browser && refreshTimer) {
      clearTimeout(refreshTimer);
      refreshTimer = null;
    }
  });
</script>

<svelte:window on:visibilitychange={() => (canRefresh = document.visibilityState === 'visible')} />

<main class="m-4">
  <header class="flex w-full justify-end gap-4">
    <DarkModeSwitch />
    <label class="flex items-center gap-2" for="show-dismissed-switch">
      <Switch id="show-dismissed-switch" bind:checked={showDismissed} />
      Show dismissed
    </label>
  </header>
  <div class="flex flex-col gap-8">
    {#each apps as { app, items } (app.id)}
      <section>
        <h2 class="mb-4 text-xl font-medium text-surface-content/75">{app.name}</h2>
        <div class="flex flex-col gap-8">
          {#each items as { id, dismissed, persistent, data } (id)}
            <article>
              <Card>
                <div slot="header" class="grid grid-cols-[1fr_auto]">
                  <div>
                    {#if data.title}
                      <h2 class="text-primary/75">
                        {#if data.url}
                          <a href={data.url} class="underline" target="_blank">
                            {data.title}
                          </a>
                        {:else}
                          {data.title}
                        {/if}
                      </h2>
                    {/if}
                    {#if data.subtitle}
                      <div class="text-sm text-surface-content/50">
                        {data.subtitle}
                      </div>
                    {/if}
                  </div>
                  {#if !persistent}
                    <form method="POST" action="?/toggle_dismissed" use:enhance>
                      <input type="hidden" name="app_id" value={app.id} />
                      <input type="hidden" name="item_id" value={id} />
                      <input type="hidden" name="current_dismissed" value={dismissed} />
                      <Button
                        type="submit"
                        color="primary"
                        variant="fill-light"
                        icon={dismissed ? mdiEmailOpenOutline : mdiEmailOutline}
                      />
                    </form>
                  {/if}
                </div>

                <div slot="contents">
                  {#if data.detail}
                    <div class="prose dark:prose-invert">{@html data.detail}</div>
                  {/if}
                </div>
              </Card>
            </article>
          {/each}
        </div>
      </section>
    {/each}
  </div>
</main>
