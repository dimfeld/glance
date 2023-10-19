export * from './app_data.js';

import { promises as fs } from 'node:fs';
import * as path from 'node:path';
import type { AppData } from './app_data.js';

import envPaths from 'env-paths';

export function baseDataDir() {
  const paths = envPaths('glance-dashboards', { suffix: '' });
  return paths.data;
}

export function appPaths(appId: string) {
  const base = baseDataDir();
  return {
    appDataFile: path.join(base, 'app_data', `${appId}.json`),
    appStateDir: path.join(base, 'app_state'),
    tmpDataDir: path.join(base, 'tmp'),
  };
}

export async function writeAppData(appId: string, appData: AppData) {
  const { appDataFile, tmpDataDir } = appPaths(appId);
  const tmpPath = path.join(tmpDataDir, `${appId}-${Date.now()}.json`);

  // Write and rename to simulate an atomic write
  await fs.writeFile(tmpPath, JSON.stringify(appData, null, 2));
  await fs.rename(tmpPath, appDataFile);
}
