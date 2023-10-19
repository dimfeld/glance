export * from './app_data.js';

import envPaths from 'env-paths';
import * as path from 'node:path';

export function appPaths(appId: string) {
  const paths = envPaths('glance-dashboards', { suffix: '' });

  return {
    appData: path.join(paths.data, 'app_data', `${appId}.json`),
    appState: path.join(paths.data, 'app_state'),
  };
}
