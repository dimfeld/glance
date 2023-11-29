<script lang="ts">
  import { newId } from '$lib/auto_id';
  import { Label } from '$lib/components/ui/label';

  export let id = newId();
  export let label: string;
</script>

<div class="flex flex-col gap-2 items-start">
  <Label for={id}><slot name="label">{label}</slot></Label>
  <slot {id} />
</div>
