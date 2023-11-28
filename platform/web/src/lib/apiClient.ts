import ky, { type Options } from 'ky';
import type { AppData, AppDataItem } from 'glance-app';

const localApiUrl = '/api';

export interface ApiClientExtraOptions {
  current?: URL | { url: URL };
}

export function apiClient(url: string, options: Options & ApiClientExtraOptions = {}) {
  let current =
    options.current && 'origin' in options.current ? options.current : options.current?.url;
  let origin = current?.origin;
  let prefix = new URL(localApiUrl, origin);

  return ky(url, {
    prefixUrl: prefix,
    ...options,
  });
}

export interface ActiveItems {
  app: AppData & { id: string };
  items: AppDataItem[];
}
