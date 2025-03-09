<script lang="ts">
  import { fade } from "svelte/transition";
  import { isSearchOpen } from "$lib/stores";
  import { page } from "$app/state";

  function close() {
    isSearchOpen.set(false);
  }
</script>

{#if $isSearchOpen}
  <button
    class="fixed inset-0 z-30 items-start justify-center bg-stone-900/80 pt-20 backdrop-blur-sm dark:bg-stone-950/80"
    transition:fade={{ duration: 100 }}
    onclick={close}
    aria-label="Search Overlay"
  ></button>
{/if}
