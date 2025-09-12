<script lang="ts">
  import { X, Search } from "lucide-svelte";

  let { q = $bindable(""), placeholder = "Search..." } = $props();
</script>

<div class="relative hidden flex-1 md:flex">
  <Search
    class="absolute top-1/2 left-3 z-10 -translate-y-1/2 text-stone-400 dark:text-stone-600"
    size={16}
  />
  <input
    type="text"
    bind:value={q}
    name="q"
    {placeholder}
    class="focus:border-accent bg-default focus:ring-accent ring-default dark:focus:border-accent dark:focus:ring-accent w-full rounded-full py-2.5 pr-10 pl-10 placeholder-stone-400 shadow-md focus:ring-2 focus:outline-none dark:bg-stone-900"
  />
  {#if q}
    <button
      onclick={() => (q = "")}
      class="absolute top-1/2 right-3 -translate-y-1/2 rounded-full p-1 text-stone-400 duration-200 hover:bg-stone-100 hover:text-stone-700 dark:hover:bg-stone-800 dark:hover:text-stone-300"
      aria-label="Clear search"
    >
      <X size={16} />
    </button>
  {/if}
</div>
