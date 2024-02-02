import { type ActiveItems, apiClient } from '$lib/apiClient';

export async function load({ fetch, url }) {
  return {
    apps: await apiClient('active_items', { fetch, current: url }).json<ActiveItems[]>(),
  };
}
