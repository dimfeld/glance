<script lang="ts">
  import { goto, invalidateAll } from '$app/navigation';
  import type { Snippet } from 'svelte';
  import { Button } from 'svelte-ux';

  const { provider, name, children } = $props<{
    provider: string;
    name: string;
    children?: Snippet;
  }>();

  let loginUrl = $derived(`/api/auth/oauth/login/${provider}`);

  function oauthLogin() {
    const loginWindow = window.open(
      `${loginUrl}?frompopup=true`,
      'oauthLogin',
      'width=600,height=400'
    );

    if (loginWindow) {
      window.addEventListener('message', function handler(event) {
        loginWindow.close();
        window.removeEventListener('message', handler);

        let data = JSON.parse(event.data);
        if (data.success) {
          invalidateAll();
          goto(data.redirectTo || '/');
        } else {
          // TODO show error message here
        }
      });
    } else {
      goto(loginUrl);
    }
  }
</script>

<form action={loginUrl} method="GET" on:submit|preventDefault={oauthLogin}>
  {#if children}
    {@render children()}
  {:else}
    <Button variant="fill-outline" rounded type="submit">Login with {name}</Button>
  {/if}
</form>
