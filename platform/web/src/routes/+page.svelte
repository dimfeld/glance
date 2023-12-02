<script lang="ts">
  import SideLabelled from '$lib/components/SideLabelled.svelte';
  import * as Card from '$lib/components/ui/card';
  import Switch from '$lib/components/ui/switch/switch.svelte';
  import { Button } from '$lib/components/ui/button';
  import { MailIcon, MailOpenIcon } from 'lucide-svelte';
  import { enhance } from '$app/forms';

  export let data;

  let showDismissed = false;

  $: apps = data.apps
    .map((app) => {
      return {
        ...app,
        items: app.items.filter((item) => showDismissed || !item.dismissed),
      };
    })
    .filter((app) => app.items.length > 0);
</script>

<main class="m-4">
  <header class="flex w-full justify-end">
    <SideLabelled label="Show dismissed" id="show_dismissed" let:id>
      <Switch {id} bind:checked={showDismissed} />
    </SideLabelled>
  </header>
  <div class="flex flex-col gap-8">
    {#each apps as { app, items } (app.id)}
      <section>
        <h2 class="mb-4 text-xl">{app.name}</h2>
        <div class="flex flex-col gap-8">
          {#each items as { id, dismissed, persistent, data } (id)}
            <article>
              <Card.Root>
                <Card.Header>
                  <div class="grid grid-cols-[1fr_auto]">
                    <div>
                      {#if data.title}
                        <Card.Title>
                          {#if data.url}
                            <a href={data.url} class="underline" target="_blank">
                              {data.title}
                            </a>
                          {:else}
                            {data.title}
                          {/if}
                        </Card.Title>
                      {/if}
                      {#if data.subtitle}
                        <Card.Description>
                          {data.subtitle}
                        </Card.Description>
                      {/if}
                    </div>
                    {#if !persistent}
                      <form method="POST" action="?/toggle_dismissed" use:enhance>
                        <input type="hidden" name="app_id" value={app.id} />
                        <input type="hidden" name="item_id" value={id} />
                        <input type="hidden" name="current_dismissed" value={dismissed} />
                        <Button type="submit" variant="outline" size="icon">
                          {#if dismissed}
                            <MailOpenIcon />
                          {:else}
                            <MailIcon />
                          {/if}
                        </Button>
                      </form>
                    {/if}
                  </div>
                </Card.Header>
                {#if data.detail}
                  <Card.Content class="prose dark:prose-invert">{@html data.detail}</Card.Content>
                {/if}
              </Card.Root>
            </article>
          {/each}
        </div>
      </section>
    {/each}
  </div>
</main>
