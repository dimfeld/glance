import { client } from 'filigree-web';

export interface GetUserInput {
  fetch: typeof fetch;
  locals: App.Locals;
}

const userPromises = new WeakMap<GetUserInput, Promise<Response>>();

/** Fetch info for the current user. This places the user at `event.locals.user`,
 * which allows subsequent calls in the same request to use the first call's result. */
export async function getUser(event: GetUserInput) {
  const existingPromise = userPromises.get(event);
  if (existingPromise) {
    return existingPromise;
  }

  const responsePromise = client({
    url: '/api/self',
    fetch: event.fetch,
    // 401 means user is not logged in
    tolerateFailure: [401],
  });
  userPromises.set(event, responsePromise);

  const response = await responsePromise;
  if (response.status === 200) {
    event.locals.user = await response.json();
  }

  return event.locals.user;
}
