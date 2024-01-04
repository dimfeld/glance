<script>
  import { ThemeInit, settings } from 'svelte-ux';
  import '../app.pcss';

  settings({});
</script>

<ThemeInit />

<div id="top" class="bg-surface-100 text-surface-content h-full min-h-screen w-full overflow-auto">
  <slot />
</div>
