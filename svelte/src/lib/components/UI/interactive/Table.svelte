<script lang="ts">
  import { X, Search } from "lucide-svelte";
  import { goto } from "$app/navigation";
  import { fade } from "svelte/transition";
  import { searchTerm } from "$lib/stores";
  import type { Student, BaseTableItem, TableConfig } from "$lib/types";
  import { EmptySpace } from "$lib/components/typography";

  interface Props {
    items: BaseTableItem[];
    config: TableConfig<BaseTableItem>;
    href: string;
    students?: Student[];
    total?: number;
    showComplete?: boolean;
  }

  let { items = [], config, href, total = items.length }: Props = $props();
</script>

<div class="w-full space-y-4">
  <!-- Search & Filter Bar -->
  <div
    class="flex flex-col gap-3 md:flex-row md:items-center md:justify-between"
  >
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
          class="focus:border-cacao-400 focus:ring-cacao-500/20 dark:focus:border-cacao-500 dark:focus:ring-cacao-500/30 w-full rounded-2xl bg-white py-2.5 pr-10 pl-10 shadow-sm ring ring-stone-300/50 placeholder:text-stone-400 focus:ring-2 focus:outline-none dark:border-stone-600/30 dark:bg-stone-950  dark:placeholder:text-stone-600"
        />
        {#if $searchTerm}
          <button
            onclick={() => searchTerm.reset()}
            class="absolute top-1/2 right-3 -translate-y-1/2 rounded-full p-1 text-stone-400 transition-colors duration-200 hover:bg-stone-100 hover:text-stone-700 dark:hover:bg-stone-800 dark:hover:text-stone-300"
            aria-label="Clear search"
          >
            <X size={16} />
          </button>
        {/if}
      </div>
    </div>
  </div>

  {#if items.length === 0}
    <EmptySpace>Nothing Here</EmptySpace>
  {:else}
    <div
      class="overflow-hidden rounded-lg bg-white shadow-sm ring ring-stone-300/50 dark:border-stone-600/30 dark:bg-stone-950 "
    >
      <div class="overflow-x-auto">
        <table class="w-full table-auto">
          <thead>
            <tr
              class="border-b border-stone-300/30 bg-stone-50/30 dark:border-stone-700 dark:bg-stone-900/30"
            >
              {#each config.columns as column, index (index)}
                <th
                  class="px-4 py-2 text-left text-sm font-semibold whitespace-nowrap text-stone-700 dark:text-stone-300"
                >
                  {column.label}
                </th>
              {/each}
            </tr>
          </thead>
          <tbody class="divide-y divide-stone-200 dark:divide-stone-800">
            {#each items as item (item.id)}
              <tr
                onclick={() => goto(`${href}/${item.id}`)}
                class="group hover:bg-cacao-50/30 dark:hover:bg-cacao-900/10 max-h-24 cursor-pointer transition-all duration-150 ease-in-out"
                in:fade|global={{ duration: 150, delay: 10 }}
              >
                {#each config.columns as column, index (index)}
                  <td
                    class="max-h-24 px-4 py-2 text-sm text-stone-600 transition-all duration-200 ease-in-out group-hover:text-stone-900 dark:text-stone-400 dark:group-hover:text-stone-200"
                  >
                    <div class="max-h-20 overflow-hidden text-ellipsis">
                      {column.formatter
                        ? column.formatter(item[column.key])
                        : item[column.key]}
                    </div>
                  </td>
                {/each}
              </tr>
            {/each}
          </tbody>
        </table>
      </div>

      <div
        class="flex items-center justify-between border-t border-stone-300/30 bg-stone-50/30 px-6 py-3 text-sm text-stone-500 dark:border-stone-600/30 dark:text-stone-400"
      >
        <div>
          Showing <span class="font-medium text-stone-700 dark:text-stone-300"
            >{items.length}</span
          >
          of
          <span class="font-medium text-stone-700 dark:text-stone-300"
            >{total}</span
          > results
        </div>
      </div>
    </div>
  {/if}
</div>
