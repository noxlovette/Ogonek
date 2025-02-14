<script lang="ts">
  import { Search } from "lucide-svelte";
  import { isSearchOpen } from "$lib/stores";

  function handleKeydown(e: KeyboardEvent) {
    if ((e.metaKey || e.ctrlKey) && e.key === "k") {
      e.preventDefault();
      isSearchOpen.set(true);
      searchElement.focus();
    }
    if (e.key === "Escape") {
      isSearchOpen.set(false);
      query = "";
      searchElement.blur();
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
  <Search class="text-milk-400 absolute top-1/2 left-3 -translate-y-1/2 " />
  <input
    type="text"
    bind:value={query}
    bind:this={searchElement}
    onclick={handleSearchClick}
    {placeholder}
    class="bg-cacao-50 dark:bg-milk-950 border-milk-200 dark:border-milk-900 dark:focus:ring-milk-700 dark:focus:placeholder:text-milk-700 focus:ring-cacao-500 focus:placeholder:text-cacao-400/70 placeholder:text-milk-500 w-full rounded-full border py-2 pr-4 pl-10
        focus:border-transparent
        focus:ring-2
        "
  />
  <div
    class="text-milk-400 dark:text-milk-800 absolute top-1/2 right-3 hidden -translate-y-1/2 items-center gap-1 text-xs md:flex"
  >
    <kbd
      class="bg-milk-100 dark:bg-milk-400 dark:border-milk-800 border-milk-300 rounded border px-1.5 py-0.5"
      >{cmdKey}</kbd
    >
    <kbd
      class="bg-milk-100 dark:bg-milk-400 dark:border-milk-800 border-milk-300 rounded border px-1.5 py-0.5"
      >K</kbd
    >
  </div>
</div>

<svelte:window onkeydown={handleKeydown} onclick={handleClick} />
