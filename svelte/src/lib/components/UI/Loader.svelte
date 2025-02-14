<script>
  import { fade } from "svelte/transition";

  import { navigating } from "$app/stores";
</script>

{#if $navigating}
  <div
    class="bg-cacao-900 bg-opacity-50 fixed top-0 left-0 z-30 flex h-full w-full items-center justify-center backdrop-blur select-none"
    transition:fade={{ duration: 300 }}
  >
    <div class="text-cacao-100 font-serif text-4xl">
      Firelight is loading...
    </div>
  </div>
{/if}
