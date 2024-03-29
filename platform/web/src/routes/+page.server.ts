import { apiClient } from '$lib/apiClient.js';

export const actions = {
  toggle_dismissed: async ({ request, fetch }) => {
    const form = await request.formData();
    const app_id = form.get('app_id');
    const item_id = form.get('item_id');
    const dismissed = form.get('current_dismissed') === 'true';

    const targetState = !dismissed;
    const action = targetState ? 'dismiss' : 'undismiss';

    await apiClient({
      url: `apps/${app_id}/items/${item_id}/${action}`,
      method: 'POST',
      fetch,
    });
  },
};
