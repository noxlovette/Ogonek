<script lang="ts">
  import { X, Search } from "lucide-svelte";
  import { searchTerm } from "$lib/stores";
</script>

<div class="flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
  <div class="flex flex-grow items-center gap-3">
    <div class="relative flex-1">
      <Search
        class="absolute top-1/2 left-3 -translate-y-1/2 text-stone-400 dark:text-stone-600"
        size={18}
      />
      <input
        type="text"
        bind:value={$searchTerm}
        placeholder="Search..."
        class="focus:border-accent focus:ring-accent dark:focus:border-accent dark:focus:ring-accent w-full rounded-full py-2.5 pr-10 pl-10 shadow-md placeholder:text-stone-400 focus:ring-2 focus:outline-none"
      />
      {#if $searchTerm}
        <button
          onclick={() => searchTerm.reset()}
          class="absolute top-1/2 right-3 -translate-y-1/2 rounded-full p-1 text-stone-400 duration-200 hover:bg-stone-100 hover:text-stone-700 dark:hover:bg-stone-800 dark:hover:text-stone-300"
          aria-label="Clear search"
        >
          <X size={16} />
        </button>
      {/if}
    </div>
  </div>
</div>
