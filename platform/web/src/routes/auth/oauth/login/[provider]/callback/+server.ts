import type { RequestHandler } from '@sveltejs/kit';
import { client, copyCookies } from 'filigree-web';

export const GET: RequestHandler = async ({ url, params, fetch }) => {
  const provider = params.provider;

  const response = await client({
    url: `/api/auth/oauth/login/${provider}/callback`,
    fetch,
    query: url.searchParams,
  });

  let message: string;
  if (response.ok) {
    message = '{success:true}';
  } else if (response.headers.get('content-type')?.includes('json')) {
    // It's already JSON so no need to decode and re-encode it.
    message = await response.text();
  } else {
    // Something went more wrong
    message = JSON.stringify({ error: response.statusText });
  }

  let headers = copyCookies(response, {
    'Content-Type': 'text/html; charset=utf-8',
  });

  return new Response(
    `
    <html>
    <head>
      <script>
        window.opener.postMessage(${message}, window.location.origin)
      </script>
      <meta http-equiv="refresh" content="3; url=/" />
    </head>
    <body>
      <noscript>You have logged in. Redirecting to the application...</noscript>
    </body>
    </html>
    `,
    {
      headers,
    }
  );
};
