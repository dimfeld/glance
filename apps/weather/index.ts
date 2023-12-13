#!/usr/bin/env bun
import { type AppData, writeAppData } from 'glance-app';
import ky from 'ky';

// Partially typed
interface WeatherApiBaseResponse {
  properties: {
    forecast: string;
    forecastHourly: string;
  };
}

interface Forecast {
  properties: {
    periods: Period[];
  };
}

interface Period {
  number: number;
  name: string;
  startTime: string;
  endTime: string;
  isDaytime: boolean;
  temperature: number;
  temperatureUnit: 'F' | 'C';
  temperatureTrend: null | 'falling' | 'rising' | 'steady';
  windSpeed: string;
  windDirection: string;
  icon: string;
  shortForecast: string;
  detailedForecast: string;
  probabilityOfPrecipitation?: {
    unitCode: string;
    value: number | null;
  };
}

const LAT = process.env.LAT || '21.96163';
const LON = process.env.LON || '-159.37478';

const baseData = await ky(
  `https://api.weather.gov/points/${LAT},${LON}`
).json<WeatherApiBaseResponse>();

const [forecast, hourly] = await Promise.all([
  ky(baseData.properties.forecast).json<Forecast>(),
  null, // ky(baseData.properties.forecastHourly).json(),
]);

const period = forecast.properties.periods[0];

const appData: AppData = {
  name: 'Weather Forecast',
  path: __filename,
  ui: {},
  schedule: [
    {
      cron: '0 */15 * * * *',
    },
  ],
  items: [
    {
      id: period.startTime,
      data: {
        title: `${period.temperature} ${period.temperatureUnit}, ${
          period.probabilityOfPrecipitation?.value ?? 0
        }% Rain`,
        subtitle: period.shortForecast,
        detail: period.detailedForecast,
      },
      updated: new Date().toISOString(),
    },
  ],
};

console.dir(appData, { depth: null });
await writeAppData('weather', appData);
