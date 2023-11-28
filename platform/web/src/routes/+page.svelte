<script lang="ts">
  import * as Card from '$lib/components/ui/card';

  export let data;
</script>

<main class="m-4">
  {#each data.apps as { app, items } (app.id)}
    {#each items as { id, data } (id)}
      <Card.Root>
        <Card.Header>
          {#if data.title}
            <Card.Title>
              {data.title}
            </Card.Title>
          {/if}
          {#if data.subtitle}
            <Card.Description>
              {data.subtitle}
            </Card.Description>
          {/if}
        </Card.Header>
        {#if data.detail}
          <Card.Content>{data.detail}</Card.Content>
        {/if}
      </Card.Root>
    {/each}
  {/each}
</main>
