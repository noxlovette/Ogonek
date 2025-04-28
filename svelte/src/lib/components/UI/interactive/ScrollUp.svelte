<script lang="ts">
  import { ChevronUp } from "lucide-svelte";

  let y = $state(0);

  function scrollToTop() {
    window.scrollTo({
      top: 0,
      behavior: "smooth",
    });
  }

  const showButton = $derived(y > 500);
</script>

<svelte:window bind:scrollY={y} />

{#if showButton}
  <button
    onclick={scrollToTop}
    class="fixed right-8 bottom-8 flex h-12 w-12 items-center justify-center
           rounded-full bg-stone-700 text-stone-100 shadow-lg transition-all
           duration-300 hover:bg-stone-800 focus:ring-2 focus:ring-stone-400 focus:outline-none"
  >
    <ChevronUp />
  </button>
{/if}
