import { logout } from 'filigree-svelte';

export async function load(event) {
  await logout(event);
  return {};
}
