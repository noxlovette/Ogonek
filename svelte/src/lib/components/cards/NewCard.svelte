<script lang="ts">
  import { Plus } from "@lucide/svelte";
  const { addCard } = $props();
</script>

<button
  data-cy="new-card"
  type="button"
  class="group focus:ring-accent flex min-h-20 w-full items-center justify-center rounded-2xl border border-dashed border-stone-200 shadow-sm transition focus:ring-2 focus:ring-offset-2 focus:outline-none dark:border-stone-800"
  onclick={addCard}
>
  <Plus class="group-hover:text-accent" />
</button>
