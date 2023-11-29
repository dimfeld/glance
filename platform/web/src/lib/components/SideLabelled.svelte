<script lang="ts">
  import { newId } from '$lib/auto_id';
  import { Label } from '$lib/components/ui/label';

  export let id = newId();
  export let label: string;
  export let side: 'left' | 'right' = 'right';
</script>

<div class="flex items-center gap-2" class:flex-row-reverse={side === 'left'}>
  <slot {id} />
  <Label for={id}><slot name="label">{label}</slot></Label>
</div>
