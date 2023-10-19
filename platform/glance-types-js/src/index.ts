export * from './app_data.js';

import { promises as fs } from 'node:fs';
import * as path from 'node:path';
import type { AppData } from './app_data.js';

import envPaths from 'env-paths';

export function appPaths(appId: string) {
  const paths = envPaths('glance-dashboards', { suffix: '' });

  return {
    appDataFile: path.join(paths.data, 'app_data', `${appId}.json`),
    tmpDataDir: path.join(paths.data, 'tmp'),
    appStateDir: path.join(paths.data, 'app_state'),
  };
}

export async function writeAppData(appId: string, appData: AppData) {
  const { appDataFile, tmpDataDir } = appPaths(appId);
  const tmpPath = path.join(tmpDataDir, `${appId}-${Date.now()}.json`);

  // Write and rename to simulate an atomic write
  await fs.writeFile(tmpPath, JSON.stringify(appData, null, 2));
  await fs.rename(tmpPath, appDataFile);
}
