import { calculateThemeFromServer } from '$lib/theme';

/** @type {import('./$types').LayoutServerLoad} */
export async function load({ cookies }) {
  return {
    theme: calculateThemeFromServer(cookies),
  };
}
