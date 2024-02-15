import { client } from 'filigree-web';

export async function load({ fetch, cookies }) {
  await client({
    url: '/api/auth/logout',
    method: 'POST',
    fetch,
  });

  cookies.delete('sid', { path: '/' });

  return {};
}
