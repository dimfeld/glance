<script>
  import { addDarkMode, createDarkStore } from '$lib/theme';
  import '../app.pcss';

  export let data;

  const dark = createDarkStore(data.theme).resolved();
</script>

<!-- Also add it on the body so that it's applied to the background with overscroll -->
<svelte:body use:addDarkMode={$dark} />

<div id="top" class="w-full min-h-screen h-full overflow-auto" class:dark={$dark === 'dark'}>
  <slot />
</div>
