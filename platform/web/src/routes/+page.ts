import { type ActiveItems, apiClient } from '$lib/apiClient';

export async function load({ fetch, url }) {
  return {
    apps: apiClient('active_items', { fetch, current: url }).json<ActiveItems[]>(),
  };
}
