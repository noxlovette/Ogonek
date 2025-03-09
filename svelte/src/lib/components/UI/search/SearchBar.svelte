<script lang="ts">
  import { Search } from "lucide-svelte";
  import { isSearchOpen } from "$lib/stores";

  function handleKeydown(e: KeyboardEvent) {
    if ((e.metaKey || e.ctrlKey) && e.key === "k") {
      e.preventDefault();
      isSearchOpen.set(true);
      searchElement.focus();
    }
  }

  function handleClick(e: MouseEvent) {
    if (searchElement && !searchElement.contains(e.target as Node)) {
      query = "";
      isSearchOpen.set(false);
      searchElement.blur();
    }
  }

  function handleSearchClick() {
    isSearchOpen.set(true);
    searchElement.focus();
  }
  let { query = $bindable(""), placeholder = "Search..." } = $props();
  let searchElement: HTMLInputElement;

  const isMac = navigator.platform.toLowerCase().includes("mac");
  const cmdKey = isMac ? "⌘" : "Ctrl";
</script>

<div class="relative flex-1">
  <Search class="absolute top-1/2 left-3 -translate-y-1/2 text-stone-400 " />
  <input
    type="text"
    bind:value={query}
    bind:this={searchElement}
    onclick={handleSearchClick}
    {placeholder}
    class="bg-cacao-50 focus:ring-cacao-500 focus:placeholder:text-cacao-400/70 w-full rounded-full border border-stone-200 py-2 pr-4 pl-10 placeholder:text-stone-500 focus:border-transparent focus:ring dark:border-stone-900 dark:bg-stone-950
        dark:focus:ring-stone-700
        dark:focus:placeholder:text-stone-700
        "
  />
  <div
    class="absolute top-1/2 right-3 hidden -translate-y-1/2 items-center gap-1 text-xs text-stone-400 md:flex dark:text-stone-800"
  >
    <kbd
      class="rounded border border-stone-300 bg-stone-100 px-1.5 py-0.5 dark:border-stone-800 dark:bg-stone-400"
      >{cmdKey}</kbd
    >
    <kbd
      class="rounded border border-stone-300 bg-stone-100 px-1.5 py-0.5 dark:border-stone-800 dark:bg-stone-400"
      >K</kbd
    >
  </div>
</div>

<svelte:window onkeydown={handleKeydown} onclick={handleClick} />
