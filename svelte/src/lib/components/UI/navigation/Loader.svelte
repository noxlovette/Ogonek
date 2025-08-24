<script>
  import { isLoading } from "$lib/stores";
  import { Loader } from "lucide-svelte";
</script>

{#if $isLoading}
  <div
    class="absolute inset-0 z-50 flex items-center justify-center bg-white/60"
  >
    <Loader class="h-10 w-10 animate-spin text-stone-700" />
  </div>
{/if}
