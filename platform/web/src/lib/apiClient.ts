import { client } from 'filigree-web';
import type { AppData, AppItem } from 'glance-app';

export interface ActiveItems {
  app: AppData & { id: string };
  items: (AppItem & { dismissed: boolean })[];
}

export const apiClient = client.extend({
  prefixUrl: '/api',
});
