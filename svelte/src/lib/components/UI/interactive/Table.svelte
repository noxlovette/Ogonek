<script lang="ts" generics="T extends BaseTableItem">
	import { enhance } from '$app/forms';
	import { PlusCircle, X } from 'lucide-svelte';
	import { Search } from 'lucide-svelte';
	import { goto } from '$app/navigation';
	import { fade } from 'svelte/transition';
	import { notification, tableQuery } from '$lib/stores';
	import type { Student, BaseTableItem, TableConfig } from '$lib/types';

	interface Props<T extends BaseTableItem> {
		items: T[];
		config: TableConfig<T>;
		href: string;
		students: Student[];
	}

	let { items, config, href, students = [] }: Props<T> = $props();

	let filteredItems = $state(items);
	let foundItems = $state(items);
	let filterAssignee = $state('');
	let isSubmitting = $state(false);

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
						? String(column.formatter(value)).toLowerCase() // Ensure formatter output is a string
						: String(value).toLowerCase();

					return searchableValue.includes(lowercaseQuery);
				});

			// Check for 'assignee' existence and match
			const matchesAssignee =
				'assignee' in item &&
				typeof item.assignee === 'string' &&
				item.assignee.toLowerCase().includes(lowercaseQuery);

			return matchesColumn || matchesAssignee;
		});
	});

	$effect(() => {
		filteredItems = filterAssignee
			? foundItems.filter((item) => 'assignee' in item && item.assignee === filterAssignee)
			: foundItems;
	});

	let sortField: keyof T = $state('id'); // Default sorting field
	let sortDirection = $state('desc'); // 'asc' or 'desc'

	function sortByColumn(field: keyof T) {
		if (sortField === field) {
			// Toggle sort direction
			sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
		} else {
			// Set new sort field
			sortField = field;
			sortDirection = 'asc';
		}

		// Apply sorting to items
		filteredItems = [...filteredItems].sort((a, b) => {
			const aValue = a[sortField];
			const bValue = b[sortField];

			// Handle null/undefined gracefully
			if (aValue == null) return sortDirection === 'asc' ? -1 : 1;
			if (bValue == null) return sortDirection === 'asc' ? 1 : -1;

			if (typeof aValue === 'string' && typeof bValue === 'string') {
				return sortDirection === 'asc'
					? aValue.localeCompare(bValue)
					: bValue.localeCompare(aValue);
			}

			if (typeof aValue === 'number' && typeof bValue === 'number') {
				return sortDirection === 'asc' ? aValue - bValue : bValue - aValue;
			}

			if (aValue instanceof Date && bValue instanceof Date) {
				return sortDirection === 'asc'
					? new Date(aValue).getTime() - new Date(bValue).getTime()
					: new Date(bValue).getTime() - new Date(aValue).getTime();
			}

			return 0; // Default to no sorting
		});
	}
</script>

<div class="w-full space-y-4">
	<!-- Search & Filter Bar -->
	<div class="flex gap-4 items-center">
		{#if items.length !== 0}
			<div class="hidden md:flex md:relative flex-1">
				<Search class="absolute left-3 top-1/2 -translate-y-1/2 text-milk-400 dark:text-milk-700" />
				<input
					type="text"
					bind:value={$tableQuery}
					placeholder="Search..."
					class="w-full pl-10 {$tableQuery ? 'pr-10' : 'pr-4'}
						w-full pl-10 pr-4 py-2 border rounded-full focus:ring-2 bg-brick-50 dark:bg-milk-950 border-milk-200 dark:border-milk-900 dark:focus:ring-milk-700 dark:focus:placeholder:text-milk-700 focus:ring-brick-500 focus:border-transparent
        focus:placeholder:text-brick-400/70
        placeholder:text-milk-500
        "
				/>
				{#if $tableQuery}
					<button
						onclick={() => ($tableQuery = '')}
						class="absolute right-3 top-1/2 -translate-y-1/2 text-milk-400 hover:text-milk-600 transition-colors duration-200"
					>
						<X size={16} />
					</button>
				{/if}
			</div>
			<div class="flex flex-row items-center gap-4">
				<form
					action="?/new"
					method="post"
					class="hidden md:flex"
					use:enhance={() => {
						isSubmitting = true;

						return async ({ result }) => {
							isSubmitting = false;
							if (result.type === 'redirect') {
								notification.set({ message: 'New entry created', type: 'success' });
								goto(result.location);
							} else {
								notification.set({
									message: "Something's off",
									type: 'error'
								});
							}
						};
					}}
				>
					<button
						class="p-1 dark:text-milk-700 dark:hover:text-milk-600 text-brick-600 hover:text-brick-500 transition-colors"
					>
						<PlusCircle class="size-8" />
					</button>
				</form>
				{#if students.length > 0}
					<select
						id="assignee"
						name="assignee"
						bind:value={filterAssignee}
						class="w-full text-xs md:text-base dark:bg-milk-950 rounded-lg dark:border-milk-900 dark:focus:ring-milk-700 border-milk-200 shadow-sm dark:focus:border-milk-800 focus:border-brick-500 focus:ring-brick-500"
					>
						<option value="">All Students</option>
						{#each students as student}
							<option value={student.id}>
								{student.name}
							</option>
						{/each}
					</select>
				{/if}
			</div>
		{/if}
	</div>

	{#if items.length === 0}
		<div class="flex text-milk-500">
			<p class="text-lg">Looks like tumbleweeds in here... 🌵</p>
		</div>
	{:else}
		<!-- Table -->
		<div class="overflow-x-auto rounded-lg border shadow-md dark:border-milk-800">
			<table class="w-full">
				<thead class="bg-milk-50 dark:bg-milk-900">
					<tr>
						{#each config.columns as column}
							<th
								class="px-6 py-3 text-left text-sm font-semibold cursor-pointer hover:underline"
								onclick={() => sortByColumn(column.key)}
							>
								{column.label}
								{#if sortField === column.key}
									{#if sortDirection === 'asc'}
										↑
									{:else}
										↓
									{/if}
								{/if}
							</th>
						{/each}
					</tr>
				</thead>
				<tbody class="divide-y divide-milk-200 bg-white dark:bg-milk-950">
					{#each filteredItems as item (item.id)}
						<tr
							onclick={() => goto(`${href}/${item.id}`)}
							class="hover:bg-milk-50 dark:hover:bg-milk-800 cursor-pointer transition-colors"
							in:fade={{ duration: 200 }}
						>
							{#each config.columns as column}
								<td class="px-6 py-4 text-sm">
									{column.formatter ? column.formatter(item[column.key]) : item[column.key]}
								</td>
							{/each}
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>
