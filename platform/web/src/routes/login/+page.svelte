<script lang="ts">
  import { manageForm } from 'filigree-web';
  import { Button, TextField } from 'svelte-ux';
  import OAuthLoginButton from '$lib/components/OAuthLoginButton.svelte';

  export let data;
  export let form;

  const { enhance, errors, formData, message, loading, slowLoading } = manageForm({
    form: form,
  });
</script>

<form class="mx-auto flex max-w-3xl flex-col gap-4" method="POST" use:enhance>
  <TextField labelPlacement="top" name="email" label="Email" />
  <TextField labelPlacement="top" name="password" label="Password" type="password" />
  <Button type="submit">Login</Button>
</form>

<OAuthLoginButton provider="github" name="GitHub" />
<OAuthLoginButton provider="twitter" name="Twitter" />
<OAuthLoginButton provider="google" name="Google" />
