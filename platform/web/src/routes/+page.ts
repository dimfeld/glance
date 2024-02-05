import { type ActiveItems, apiClient } from '$lib/apiClient';

export async function load({ fetch }) {
  return {
    apps: await apiClient({ url: 'active_items', fetch }).json<ActiveItems[]>(),
  };
}
