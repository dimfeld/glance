<script lang="ts">
  export let data;
</script>

{#each data.apps as { app, items } (app.app.app_id)}
  {#each items as item (item.id)}
    <div>
      {@html item.html}
    </div>
  {/each}
{/each}
