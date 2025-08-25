<script>
  import { isLoading } from "$lib/stores";
  import { Loader } from "lucide-svelte";
</script>

{#if $isLoading}
  <Loader
    class="absolute top-2 right-2 size-12 animate-spin text-stone-700 dark:text-stone-200"
  />
{/if}
