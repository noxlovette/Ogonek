<script lang="ts">
  import { clickOutside } from "$lib/actions";
  import { Settings2 } from "@lucide/svelte";

  let isOpen = $state(false);

  function togglePopover() {
    isOpen = !isOpen;
  }

  function closePopover() {
    isOpen = false;
  }

  const { children } = $props();
</script>

<div class="relative inline-block">
  <button
    type="button"
    onclick={togglePopover}
    class="input-default ring-default inline-flex w-full gap-1"
    aria-expanded={isOpen}
    aria-haspopup="true"
  >
    <Settings2 color={isOpen ? "#df7055" : "currentColor"} />
  </button>

  {#if isOpen}
    <div
      class="bg-solid gap-default ring-default absolute top-full right-0 z-40 mt-2 flex min-w-[280px]
             flex-col rounded-2xl border-stone-200 p-2.5 py-4 shadow-lg"
    >
      {@render children?.()}
    </div>
  {/if}
</div>
