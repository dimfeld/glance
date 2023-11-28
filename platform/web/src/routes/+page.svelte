<script lang="ts">
  export let data;
</script>

<main class="m-4">
  {#each data.apps as { app, items } (app.id)}
    {#each items as { id, data } (id)}
      <div class="w-full border border-gray-200 p-4">
        <p>{data.title}</p>
        <p>{data.subtitle}</p>
        <p>{data.detail}</p>
      </div>
    {/each}
  {/each}
</main>
