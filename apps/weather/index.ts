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

const baseData = await ky(
  `https://api.weather.gov/points/${process.env.LAT},${process.env.LON}`
).json<WeatherApiBaseResponse>();

const [forecast, hourly] = await Promise.all([
  ky(baseData.properties.forecast).json<Forecast>(),
  null, // ky(baseData.properties.forecastHourly).json(),
]);

console.dir(forecast, { depth: null });

const period = forecast.properties.periods[0];
const output = `${period.temperature} ${period.temperatureUnit}, <span title="${Bun.escapeHTML(
  period.detailedForecast
)}">${Bun.escapeHTML(period.shortForecast)}</span>, ${
  period.probabilityOfPrecipitation?.value ?? 0
}% Rain`;

const appData: AppData = {
  name: 'Weather Forecast',
  path: __filename,
  schedule: [
    {
      cron: '*/15 * * * *',
    },
  ],
  items: [
    {
      id: period.startTime,
      html: Bun.escapeHTML(period.detailedForecast),
      updated: new Date().toISOString(),
    },
  ],
};

await writeAppData('weather', appData);
