<script lang="ts">
  import { mdiWeatherNight, mdiWeatherSunny } from '@mdi/js';
  import { getSettings, Icon } from 'svelte-ux';

  const { currentTheme } = getSettings();

  let dark = $derived($currentTheme.dark);

  function toggleDark() {
    currentTheme.setTheme(dark ? 'light' : 'dark');
  }
</script>

<button
  type="button"
  class="grid h-6 w-12 overflow-clip rounded-full border border-neutral-500"
  role="checkbox"
  aria-checked={dark}
  on:click={toggleDark}
>
  <div
    class="ls-button relative grid h-6 w-6 transform grid-cols-1 grid-rows-1 place-items-center bg-surface-100 transition-transform duration-300 dark:translate-x-5"
  >
    <Icon
      classes={{
        root: 'ls-icon w-5 h-5 transition-opacity duration-200 opacity-0 dark:opacity-100',
      }}
      path={mdiWeatherNight}
    />
    <Icon
      classes={{
        root: 'ls-icon w-5 h-5 transition-opacity duration-200 dark:opacity-0 opacity-100',
      }}
      path={mdiWeatherSunny}
    />
  </div>
</button>

<!--
<Button
  color="default"
  variant="fill-outline"
  role="checkbox"
  aria-checked={dark}
  on:click={toggleDark}
  icon={mdiThemeLightDark}
/>
-->

<style>
  .ls-button :global(.ls-icon) {
    grid-row: 1;
    grid-column: 1;
  }
</style>
