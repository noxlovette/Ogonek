<script lang="ts">
  import { goto } from "$app/navigation";
  import { fade } from "svelte/transition";
  import type { Student, TableConfig } from "$lib/types";
  import { EmptySpace } from "$lib/components/typography";
  import SearchBar from "./SearchBar.svelte";

  interface Props {
    items: any[];
    config: TableConfig<any>;
    href: string;
    students?: Student[];
    total?: number;
    showComplete?: boolean;
  }

  let { items, config, href, total = items.length }: Props = $props();
</script>

<div class="w-full space-y-4">
  <!-- Search & Filter Bar -->
  <SearchBar />

  {#if items.length === 0}
    <EmptySpace>Nothing Here</EmptySpace>
  {:else}
    <div
      class="ring-default overflow-hidden rounded-lg bg-white shadow-sm dark:border-stone-600/30 dark:bg-stone-950"
    >
      <div class="overflow-x-auto">
        <table class="w-full table-auto">
          <thead>
            <tr
              class="bg-default border-b border-stone-300/30 dark:border-stone-700 dark:bg-stone-900/30"
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
        class="bg-default flex items-center justify-between border-t border-stone-300/30 px-6 py-3 text-sm text-stone-500 dark:border-stone-600/30 dark:text-stone-400"
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
