<script lang="ts">
  import { Plus } from "lucide-svelte";
  const { addCard } = $props();
</script>

<button
  type="button"
  class="group focus:ring-accent flex h-full w-full items-center justify-center rounded-2xl border border-dashed border-stone-200 shadow-sm transition focus:ring-2 focus:ring-offset-2 focus:outline-none dark:border-stone-800"
  onclick={addCard}
>
  <Plus class="group-hover:text-accent" />
</button>
