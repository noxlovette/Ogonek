<script lang="ts" generics="T extends BaseTableItem">
  import { enhance } from "$app/forms";
  import { PlusCircle, X, Search, LucideEye, RotateCcw } from "lucide-svelte";
  import { goto } from "$app/navigation";
  import { fade } from "svelte/transition";
  import { enhanceForm } from "$lib/utils";
  import {
    searchTerm,
    pageSize,
    currentPage,
    assigneeStore,
    completedStore,
  } from "$lib/stores";
  import type { Student, BaseTableItem, TableConfig } from "$lib/types";
  import { page } from "$app/state";
  import UniButton from "../UniButton.svelte";

  interface Props<T extends BaseTableItem> {
    items: T[];
    config: TableConfig<T>;
    href: string;
    students?: Student[];
    total?: number;
    showComplete?: boolean;
  }

  let {
    items = $bindable([]),
    config,
    href,
    students = [],
    total = items.length,
    showComplete = false,
  }: Props<T> = $props();

  const isEmptySearch = $derived(
    items.length === 0 && page.url.searchParams.has("search"),
  );

  function resetFilters() {
    searchTerm.reset();
    pageSize.reset();
    currentPage.reset();
    assigneeStore.reset();
  }
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
          class="focus:border-cacao-400 focus:ring-cacao-500/20 dark:focus:border-cacao-500 dark:focus:ring-cacao-500/30 w-full rounded-full border border-stone-200 bg-white py-2.5 pr-10 pl-10 shadow-sm placeholder:text-stone-400 focus:ring-2 focus:outline-none dark:border-stone-800 dark:bg-stone-950 dark:placeholder:text-stone-600"
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
    {#if items.length !== 0}
      <div class="hidden items-center gap-3 md:flex">
        {#if students.length > 0}
          <div class="relative min-w-40">
            <select
              id="assignee"
              name="assignee"
              bind:value={$assigneeStore}
              class="focus:border-cacao-500 focus:ring-cacao-500/20 dark:focus:border-cacao-500 dark:focus:ring-cacao-500/30 w-full appearance-none rounded-lg border border-stone-200 bg-white py-2 pr-10 pl-4 text-sm shadow-sm focus:ring-2 focus:outline-none dark:border-stone-800 dark:bg-stone-950"
            >
              <option value="">All Students</option>
              {#each students as student}
                <option value={student.id}>{student.name}</option>
              {/each}
            </select>
            <div
              class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-stone-500"
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
          use:enhance={enhanceForm({
            messages: {
              redirect: "New Entity Created",
              defaultError: "Something's off",
            },
            navigate: true,
          })}
        >
          <UniButton
            Icon={PlusCircle}
            iconPosition="right"
            type="submit"
            variant="primary">New</UniButton
          >
        </form>
      </div>
    {/if}
    {#if showComplete}
      <UniButton
        Icon={LucideEye}
        variant="outline"
        onclick={() => completedStore.toggle()}
        >{$completedStore === true
          ? "Hide Completed"
          : "Show Completed"}</UniButton
      >
    {/if}
  </div>

  <!-- Table Container -->
  <div
    class="overflow-hidden rounded-lg border border-stone-200 bg-white shadow-sm dark:border-stone-800 dark:bg-stone-950"
  >
    {#if items.length === 0 && !isEmptySearch}
      <!-- Empty state -->
      <div class="flex flex-col items-center justify-center p-8 text-center">
        <div class="mb-4 rounded-full bg-stone-100 p-4 dark:bg-stone-800">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-8 w-8 text-stone-500 dark:text-stone-400"
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
        <h3 class="text-lg font-medium text-stone-700 dark:text-stone-300">
          Nothing here yet
        </h3>
        <p class="mt-1 max-w-md text-stone-500 dark:text-stone-400">
          Looks like tumbleweeds in here... 🌵
        </p>
        <form
          action="?/new"
          method="post"
          class="mt-6"
          use:enhance={enhanceForm({
            messages: {
              redirect: "New Entity Created",
              defaultError: "Something's off",
            },
            navigate: true,
          })}
        >
          <UniButton
            Icon={PlusCircle}
            iconPosition="right"
            type="submit"
            variant="primary">New</UniButton
          >
        </form>
      </div>
    {:else if isEmptySearch}
      <!-- No search results -->
      <div class="flex flex-col items-center justify-center p-8 text-center">
        <div class="mb-4 rounded-full bg-stone-100 p-4 dark:bg-stone-800">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-8 w-8 text-stone-500 dark:text-stone-400"
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
        <h3 class="text-lg font-medium text-stone-700 dark:text-stone-300">
          No results found
        </h3>
        <p class="mt-1 max-w-md text-stone-500 dark:text-stone-400">
          Try adjusting your search or filters to find what you're looking for.
        </p>

        <UniButton
          Icon={RotateCcw}
          onclick={() => resetFilters()}
          variant="outline">Clear filters</UniButton
        >
      </div>
    {:else}
      <div class="overflow-x-auto">
        <table class="w-full table-auto">
          <thead>
            <tr
              class="border-b border-stone-200 bg-stone-50 dark:border-stone-700 dark:bg-stone-900"
            >
              {#each config.columns as column}
                <th
                  class="px-6 py-4 text-left text-sm font-medium whitespace-nowrap text-stone-700 dark:text-stone-300"
                >
                  <div
                    class="hover:text-cacao-600 dark:hover:text-cacao-400 inline-flex cursor-pointer items-center gap-1.5"
                  >
                    {column.label}
                  </div>
                </th>
              {/each}
            </tr>
          </thead>
          <tbody class="divide-y divide-stone-200 dark:divide-stone-800">
            {#each items as item (item.id)}
              <tr
                onclick={() => goto(`${href}/${item.id}`)}
                class="group hover:bg-cacao-50/30 dark:hover:bg-cacao-900/10 cursor-pointer transition-all duration-300 ease-in-out"
                in:fade|global={{ duration: 300, delay: 50 }}
              >
                {#each config.columns as column, i}
                  <td
                    class="px-6 py-4 text-sm text-stone-600 transition-all duration-200 ease-in-out group-hover:text-stone-900 dark:text-stone-400 dark:group-hover:text-stone-200"
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
        class="flex items-center justify-between border-t border-stone-200 bg-stone-50 px-6 py-3 text-sm text-stone-500 dark:border-stone-800 dark:bg-stone-900 dark:text-stone-400"
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
    {/if}
  </div>
</div>
