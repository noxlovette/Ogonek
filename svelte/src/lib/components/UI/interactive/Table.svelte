<script lang="ts" generics="T extends BaseTableItem">
  import { enhance } from "$app/forms";
  import {
    PlusCircle,
    X,
    Search,
    ArrowUp,
    ArrowDown,
    Filter,
  } from "lucide-svelte";
  import { goto } from "$app/navigation";
  import { fade, fly } from "svelte/transition";
  import { notification, tableQuery } from "$lib/stores";
  import type { Student, BaseTableItem, TableConfig } from "$lib/types";

  interface Props<T extends BaseTableItem> {
    items: T[];
    config: TableConfig<T>;
    href: string;
    students: Student[];
  }

  let { items, config, href, students = [] }: Props<T> = $props();

  let filteredItems = $state(items);
  let foundItems = $state(items);
  let filterAssignee = $state("");
  let isSubmitting = $state(false);
  let showMobileFilters = $state(false);
  let showEmptyMessage = $derived(
    filteredItems.length === 0 && items.length > 0,
  );

  $effect(() => {
    const lowercaseQuery = $tableQuery.toLowerCase();

    foundItems = items.filter((item) => {
      // Search through all configured columns
      const matchesColumn = config.columns
        .filter((column) => column.searchable !== false)
        .some((column) => {
          const value = item[column.key];
          // Safely handle non-string values using the formatter or convert to string
          const searchableValue = column.formatter
            ? String(column.formatter(value)).toLowerCase()
            : String(value).toLowerCase();

          return searchableValue.includes(lowercaseQuery);
        });

      // Check for 'assignee' existence and match
      const matchesAssignee =
        "assignee" in item &&
        typeof item.assignee === "string" &&
        item.assignee.toLowerCase().includes(lowercaseQuery);

      return matchesColumn || matchesAssignee;
    });
  });

  $effect(() => {
    filteredItems = filterAssignee
      ? foundItems.filter(
          (item) => "assignee" in item && item.assignee === filterAssignee,
        )
      : foundItems;
  });

  let sortField: keyof T = $state("id"); // Default sorting field
  let sortDirection = $state("desc"); // 'asc' or 'desc'

  function sortByColumn(field: keyof T) {
    if (sortField === field) {
      // Toggle sort direction
      sortDirection = sortDirection === "asc" ? "desc" : "asc";
    } else {
      // Set new sort field
      sortField = field;
      sortDirection = "asc";
    }

    // Apply sorting to items
    filteredItems = [...filteredItems].sort((a, b) => {
      const aValue = a[sortField];
      const bValue = b[sortField];

      // Handle null/undefined gracefully
      if (aValue == null) return sortDirection === "asc" ? -1 : 1;
      if (bValue == null) return sortDirection === "asc" ? 1 : -1;

      if (typeof aValue === "string" && typeof bValue === "string") {
        return sortDirection === "asc"
          ? aValue.localeCompare(bValue)
          : bValue.localeCompare(aValue);
      }

      if (typeof aValue === "number" && typeof bValue === "number") {
        return sortDirection === "asc" ? aValue - bValue : bValue - aValue;
      }

      if (aValue instanceof Date && bValue instanceof Date) {
        return sortDirection === "asc"
          ? new Date(aValue).getTime() - new Date(bValue).getTime()
          : new Date(bValue).getTime() - new Date(aValue).getTime();
      }

      return 0; // Default to no sorting
    });
  }

  function resetFilters() {
    $tableQuery = "";
    filterAssignee = "";
  }
</script>

<div class="w-full space-y-4">
  <!-- Search & Filter Bar -->
  <div
    class="flex flex-col gap-3 md:flex-row md:items-center md:justify-between"
  >
    {#if items.length !== 0}
      <div class="flex flex-1 items-center gap-3">
        <div class="relative flex-1">
          <Search
            class="text-milk-400 dark:text-milk-600 absolute top-1/2 left-3 -translate-y-1/2"
            size={18}
          />
          <input
            type="text"
            bind:value={$tableQuery}
            placeholder="Search..."
            class="border-milk-200 placeholder:text-milk-400 focus:border-cacao-400 focus:ring-cacao-500/20 dark:border-milk-800 dark:bg-milk-950 dark:placeholder:text-milk-600 dark:focus:border-cacao-500 dark:focus:ring-cacao-500/30 w-full rounded-full border bg-white py-2.5 pr-10 pl-10 shadow-sm focus:ring-2 focus:outline-none"
          />
          {#if $tableQuery}
            <button
              onclick={() => ($tableQuery = "")}
              class="text-milk-400 hover:bg-milk-100 hover:text-milk-700 dark:hover:bg-milk-800 dark:hover:text-milk-300 absolute top-1/2 right-3 -translate-y-1/2 rounded-full p-1 transition-colors duration-200"
              aria-label="Clear search"
            >
              <X size={16} />
            </button>
          {/if}
        </div>

        <!-- Filter icon for mobile -->
        <button
          class="border-milk-200 text-milk-600 hover:bg-milk-50 dark:border-milk-800 dark:bg-milk-900 dark:text-milk-400 dark:hover:bg-milk-800 inline-flex items-center justify-center rounded-full border bg-white p-2.5 shadow-sm md:hidden"
          onclick={() => (showMobileFilters = !showMobileFilters)}
          aria-label="Toggle filters"
        >
          <Filter size={18} />
        </button>
      </div>

      <!-- Controls for desktop -->
      <div class="hidden items-center gap-3 md:flex">
        {#if students.length > 0}
          <div class="relative min-w-40">
            <select
              id="assignee"
              name="assignee"
              bind:value={filterAssignee}
              class="border-milk-200 focus:border-cacao-500 focus:ring-cacao-500/20 dark:border-milk-800 dark:bg-milk-950 dark:focus:border-cacao-500 dark:focus:ring-cacao-500/30 w-full appearance-none rounded-lg border bg-white py-2 pr-10 pl-4 text-sm shadow-sm focus:ring-2 focus:outline-none"
            >
              <option value="">All Students</option>
              {#each students as student}
                <option value={student.id}>
                  {student.name}
                </option>
              {/each}
            </select>
            <div
              class="text-milk-500 pointer-events-none absolute inset-y-0 right-0 flex items-center px-2"
            >
              <svg class="h-4 w-4 fill-current" viewBox="0 0 20 20">
                <path
                  d="M7 7l3-3 3 3m0 6l-3 3-3-3"
                  stroke="currentColor"
                  stroke-width="1.5"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  fill="none"
                ></path>
              </svg>
            </div>
          </div>
        {/if}

        <form
          action="?/new"
          method="post"
          use:enhance={() => {
            isSubmitting = true;

            return async ({ result }) => {
              isSubmitting = false;
              if (result.type === "redirect") {
                notification.set({
                  message: "New entry created",
                  type: "success",
                });
                goto(result.location);
              } else {
                notification.set({
                  message: "Something's off",
                  type: "error",
                });
              }
            };
          }}
        >
          <button
            class="bg-cacao-500 hover:bg-cacao-600 focus:ring-cacao-500 dark:bg-cacao-600 dark:hover:bg-cacao-700 dark:focus:ring-offset-milk-900 inline-flex items-center justify-center gap-2 rounded-lg px-4 py-2 font-medium text-white shadow-sm transition-colors focus:ring-2 focus:ring-offset-2 focus:outline-none disabled:opacity-50"
            disabled={isSubmitting}
            aria-label="Create new item"
          >
            <PlusCircle class="size-5" />
            <span>New</span>
          </button>
        </form>
      </div>
    {/if}
  </div>

  <!-- Mobile filters (collapsible) -->
  {#if showMobileFilters}
    <div class="md:hidden" in:fly={{ y: -20, duration: 200 }}>
      <div
        class="border-milk-200 dark:border-milk-800 dark:bg-milk-900 rounded-lg border bg-white p-4 shadow-sm"
      >
        <h3 class="text-milk-700 dark:text-milk-300 mb-3 font-medium">
          Filters
        </h3>
        <div class="space-y-3">
          {#if students.length > 0}
            <div>
              <label
                for="mobile-assignee"
                class="text-milk-600 dark:text-milk-400 block text-sm"
                >Student</label
              >
              <select
                id="mobile-assignee"
                name="assignee"
                bind:value={filterAssignee}
                class="border-milk-200 focus:border-cacao-500 focus:ring-cacao-500/20 dark:border-milk-800 dark:bg-milk-950 dark:focus:border-cacao-500 dark:focus:ring-cacao-500/30 mt-1 w-full rounded-lg border bg-white py-2 pl-3 text-sm shadow-sm focus:ring-2 focus:outline-none"
              >
                <option value="">All Students</option>
                {#each students as student}
                  <option value={student.id}>
                    {student.name}
                  </option>
                {/each}
              </select>
            </div>
          {/if}
          <div class="flex justify-between pt-2">
            <button
              onclick={() => resetFilters()}
              class="text-milk-600 hover:text-milk-800 dark:text-milk-400 dark:hover:text-milk-200 text-sm"
            >
              Reset filters
            </button>
            <button
              onclick={() => (showMobileFilters = false)}
              class="bg-cacao-500 hover:bg-cacao-600 dark:bg-cacao-600 dark:hover:bg-cacao-700 rounded-lg px-3 py-1.5 text-sm font-medium text-white shadow-sm transition-colors"
            >
              Apply
            </button>
          </div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Table Container -->
  <div
    class="border-milk-200 dark:border-milk-800 dark:bg-milk-950 overflow-hidden rounded-xl border bg-white shadow-md"
  >
    {#if items.length === 0}
      <div class="flex flex-col items-center justify-center p-8 text-center">
        <div class="bg-milk-100 dark:bg-milk-800 mb-4 rounded-full p-4">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="text-milk-500 dark:text-milk-400 h-8 w-8"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4"
            />
          </svg>
        </div>
        <h3 class="text-milk-700 dark:text-milk-300 text-lg font-medium">
          Nothing here yet
        </h3>
        <p class="text-milk-500 dark:text-milk-400 mt-1 max-w-md">
          Looks like tumbleweeds in here... 🌵
        </p>
        <form
          action="?/new"
          method="post"
          class="mt-6"
          use:enhance={() => {
            isSubmitting = true;
            return async ({ result }) => {
              isSubmitting = false;
              if (result.type === "redirect") {
                notification.set({
                  message: "New entry created",
                  type: "success",
                });
                goto(result.location);
              } else {
                notification.set({
                  message: "Something's off",
                  type: "error",
                });
              }
            };
          }}
        >
          <button
            class="bg-cacao-500 hover:bg-cacao-600 focus:ring-cacao-500 dark:bg-cacao-600 dark:hover:bg-cacao-700 dark:focus:ring-offset-milk-900 inline-flex items-center justify-center gap-2 rounded-lg px-4 py-2 font-medium text-white shadow-sm transition-colors focus:ring-2 focus:ring-offset-2 focus:outline-none disabled:opacity-50"
            disabled={isSubmitting}
          >
            <PlusCircle class="size-5" />
            <span>Create new</span>
          </button>
        </form>
      </div>
    {:else if showEmptyMessage}
      <div class="flex flex-col items-center justify-center p-8 text-center">
        <div class="bg-milk-100 dark:bg-milk-800 mb-4 rounded-full p-4">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="text-milk-500 dark:text-milk-400 h-8 w-8"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
        </div>
        <h3 class="text-milk-700 dark:text-milk-300 text-lg font-medium">
          No results found
        </h3>
        <p class="text-milk-500 dark:text-milk-400 mt-1 max-w-md">
          Try adjusting your search or filters to find what you're looking for.
        </p>
        <button
          onclick={() => resetFilters()}
          class="border-milk-300 text-milk-700 hover:bg-milk-50 focus:ring-cacao-500 dark:border-milk-700 dark:bg-milk-800 dark:text-milk-300 dark:hover:bg-milk-700 dark:focus:ring-offset-milk-900 mt-6 inline-flex items-center justify-center rounded-lg border bg-white px-4 py-2 text-sm font-medium shadow-sm transition-colors focus:ring-2 focus:ring-offset-2 focus:outline-none"
        >
          Clear filters
        </button>
      </div>
    {:else}
      <div class="overflow-x-auto">
        <table class="w-full table-auto">
          <thead>
            <tr
              class="border-milk-200 bg-milk-50 dark:border-milk-700 dark:bg-milk-900 border-b"
            >
              {#each config.columns as column}
                <th
                  class="text-milk-700 dark:text-milk-300 px-6 py-4 text-left text-sm font-medium whitespace-nowrap"
                  onclick={() => sortByColumn(column.key)}
                >
                  <div
                    class="hover:text-cacao-600 dark:hover:text-cacao-400 inline-flex cursor-pointer items-center gap-1.5"
                  >
                    {column.label}
                    {#if sortField === column.key}
                      <span class="text-cacao-500 dark:text-cacao-400">
                        {#if sortDirection === "asc"}
                          <ArrowUp size={16} />
                        {:else}
                          <ArrowDown size={16} />
                        {/if}
                      </span>
                    {/if}
                  </div>
                </th>
              {/each}
            </tr>
          </thead>
          <tbody class="divide-milk-200 dark:divide-milk-800 divide-y">
            {#each filteredItems as item (item.id)}
              <tr
                onclick={() => goto(`${href}/${item.id}`)}
                class="group hover:bg-cacao-50/30 dark:hover:bg-cacao-900/10 cursor-pointer transition-colors"
                in:fade={{ duration: 200 }}
              >
                {#each config.columns as column}
                  <td
                    class="text-milk-600 group-hover:text-milk-900 dark:text-milk-400 dark:group-hover:text-milk-200 px-6 py-4 text-sm"
                  >
                    {column.formatter
                      ? column.formatter(item[column.key])
                      : item[column.key]}
                  </td>
                {/each}
              </tr>
            {/each}
          </tbody>
        </table>
      </div>

      <!-- Table footer with row count -->
      <div
        class="border-milk-200 bg-milk-50 text-milk-500 dark:border-milk-800 dark:bg-milk-900 dark:text-milk-400 flex items-center justify-between border-t px-6 py-3 text-sm"
      >
        <div>
          Showing <span class="text-milk-700 dark:text-milk-300 font-medium"
            >{filteredItems.length}</span
          >
          of
          <span class="text-milk-700 dark:text-milk-300 font-medium"
            >{items.length}</span
          > results
        </div>

        <!-- Mobile create button -->
        <form
          action="?/new"
          method="post"
          class="md:hidden"
          use:enhance={() => {
            isSubmitting = true;
            return async ({ result }) => {
              isSubmitting = false;
              if (result.type === "redirect") {
                notification.set({
                  message: "New entry created",
                  type: "success",
                });
                goto(result.location);
              } else {
                notification.set({
                  message: "Something's off",
                  type: "error",
                });
              }
            };
          }}
        >
          <button
            class="bg-cacao-500 hover:bg-cacao-600 focus:ring-cacao-500 dark:bg-cacao-600 dark:hover:bg-cacao-700 dark:focus:ring-offset-milk-900 inline-flex items-center justify-center rounded-lg px-3 py-1.5 text-sm font-medium text-white shadow-sm transition-colors focus:ring-2 focus:ring-offset-2 focus:outline-none disabled:opacity-50"
            disabled={isSubmitting}
          >
            <PlusCircle class="mr-1.5 h-4 w-4" />
            New
          </button>
        </form>
      </div>
    {/if}
  </div>
</div>
