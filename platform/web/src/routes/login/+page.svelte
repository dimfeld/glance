<script lang="ts">
  import { manageForm } from 'filigree-web';
  import { Button, TextField } from 'svelte-ux';
  import { page } from '$app/stores';
  import OAuthLoginButton from '$lib/components/OAuthLoginButton.svelte';

  const { data, form } = $props();

  let formManager = manageForm({
    form,
    onSuccess(result) {
      // Redirect means we logged in with a password, so we want to fetch the user again.
      // Normal success means passwordless login, and so we aren't actually logged in yet.
      return {
        invalidateAll: result.type === 'redirect',
      };
    },
  });

  let { errors, message, loading, formData, slowLoading } = $derived(formManager);
  let { enhance } = formManager;
  function handleMessage(m: string) {
    formManager.message = m;
  }
</script>

<div class="mx-auto flex max-w-lg flex-col gap-8">
  <form
    class="flex flex-col gap-4"
    method="POST"
    action={formData.password ? '?/login' : '?/passwordless'}
    use:enhance
  >
    <input
      type="hidden"
      name="redirect_to"
      value={$page.url.searchParams.get('redirectTo') || '/'}
    />
    <TextField labelPlacement="top" name="email" label="Email" bind:value={formData.email} />
    <TextField
      labelPlacement="top"
      name="password"
      label="Password"
      type="password"
      bind:value={formData.password}
    />
    <Button variant="fill" color="primary" type="submit">
      {#if formData.password}
        Login with Password
      {:else}
        Email me a Login Link
      {/if}
    </Button>
    <p class="text-sm">
      {#if message}
        {message}
      {:else}
        Type a password, or leave it blank to receive an email with a login link.
      {/if}
    </p>
  </form>

  <div class="flex w-full flex-col items-stretch gap-2">
    {#if data.oauthEnabled?.github}
      <OAuthLoginButton provider="github" name="GitHub" onMessage={handleMessage} />
    {/if}
    {#if data.oauthEnabled?.twitter}
      <OAuthLoginButton provider="twitter" name="Twitter" onMessage={handleMessage} />
    {/if}
    {#if data.oauthEnabled?.google}
      <OAuthLoginButton provider="google" name="Google" onMessage={handleMessage} />
    {/if}
  </div>
</div>
