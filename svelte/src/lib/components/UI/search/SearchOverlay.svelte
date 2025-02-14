<script lang="ts">
  import { fade } from "svelte/transition";
  import { isSearchOpen } from "$lib/stores";

  function close() {
    isSearchOpen.set(false);
  }
</script>

{#if $isSearchOpen}
  <button
    class="dark:bg-milk-950/80 bg-milk-900/80 fixed inset-0 z-30 items-start justify-center pt-20 backdrop-blur-sm"
    transition:fade={{ duration: 100 }}
    onclick={close}
    aria-label="Search Overlay"
  ></button>
{/if}
