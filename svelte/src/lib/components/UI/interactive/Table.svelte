<script lang="ts">
  import { goto } from "$app/navigation";
  import { fade, fly } from "svelte/transition";
  import type { Student, TableConfig } from "$lib/types";
  import { EmptySpace } from "$lib/components/typography";

  interface Props {
    items: any[];
    config: TableConfig<any>;
    href: string;
    students?: Student[];
    total?: number;
    showComplete?: boolean;
    loading?: boolean;
    currentPage?: number;
    pageSize?: number;
    sortBy?: string;
    sortOrder?: "asc" | "desc";
  }

  let {
    items,
    config,
    href,
    total = items.length,
    currentPage = 1,
    pageSize = 20,
  }: Props = $props();

  const startItem = $derived((currentPage - 1) * pageSize + 1);
  const endItem = $derived(Math.min(currentPage * pageSize, total));

  function handleRowClick(item: any, event: KeyboardEvent | MouseEvent) {
    if (
      event instanceof KeyboardEvent &&
      event.key !== "Enter" &&
      event.key !== " "
    ) {
      return;
    }
    goto(`${href}/${item.id}`);
  }
</script>

<div class="w-full space-y-4">
  <div
    class="ring-default overflow-hidden rounded-lg bg-white shadow-sm ring-1 ring-stone-200 dark:border-stone-600/30 dark:bg-stone-950 dark:ring-stone-800"
    in:fly={{ y: 10, duration: 300 }}
  >
    <div class="overflow-x-auto">
      <table class="w-full table-auto" aria-label="Data table">
        <thead>
          <tr
            class="border-b border-stone-300/30 bg-stone-50 dark:border-stone-700 dark:bg-stone-900/30"
          >
            {#each config.columns as column, index (index)}
              <th
                class="group px-4 py-3 text-left text-sm font-semibold text-stone-700 dark:text-stone-300"
                tabindex={-1}
                role="columnheader"
                aria-sort="none"
              >
                <div class="flex items-center space-x-2">
                  <span class="whitespace-nowrap">{column.label}</span>
                </div>
              </th>
            {/each}
          </tr>
        </thead>
        <tbody class="divide-y divide-stone-200 dark:divide-stone-800">
          {#each items as item, rowIndex (item.id)}
            <tr
              class="group cursor-pointer transition-all duration-150 ease-in-out hover:bg-amber-50/50 dark:hover:bg-amber-900/10"
              onclick={(e) => handleRowClick(item, e)}
              onkeydown={(e) => handleRowClick(item, e)}
              tabindex="0"
              role="button"
              aria-label="View details for {item.name || item.title || 'item'}"
              in:fade|global={{ duration: 150, delay: rowIndex * 10 }}
            >
              {#each config.columns as column, index (index)}
                <td
                  class="px-4 py-3 text-sm text-stone-600 transition-all duration-200 ease-in-out group-hover:text-stone-900 dark:text-stone-400 dark:group-hover:text-stone-200"
                >
                  <div class="max-h-20 overflow-hidden text-ellipsis">
                    {#if column.formatter}
                      {@html column.formatter(item[column.key])}
                    {:else}
                      {item[column.key] || "—"}
                    {/if}
                  </div>
                </td>
              {/each}
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
</div>
