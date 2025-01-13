<script lang="ts" generics="T extends BaseTableItem">
	import { enhance } from '$app/forms';
	import { PlusCircle } from 'lucide-svelte';
	import SearchBar from '$lib/components/UI/SearchBar.svelte';
	import { goto } from '$app/navigation';
	import { fade } from 'svelte/transition';
	import type { Student, BaseTableItem, TableConfig } from '$lib/types';

	interface Props<T extends BaseTableItem> {
		items: T[];
		config: TableConfig<T>;
		href: string;
		students: Student[];
	}

	let { items, config, href, students }: Props<T> = $props();

	let searchQuery = $state('');
	let filteredItems = $state(items);
	let foundItems = $state(items);
	let filterAssignee = $state('');

	$effect(() => {
		const lowercaseQuery = searchQuery.toLowerCase();

		foundItems = items.filter((item) => {
			// Search through all configured columns
			return (
				config.columns
					.filter((column) => column.searchable !== false)
					.some((column) => {
						const value = item[column.key];
						// Handle non-string values using the formatter or convert to string
						const searchableValue = column.formatter
							? column.formatter(value).toLowerCase()
							: String(value).toLowerCase();

						return searchableValue.includes(lowercaseQuery);
					}) ||
				// Always include assignee in search (since it's part of BaseTableItem)
				item.assignee.toLowerCase().includes(lowercaseQuery)
			);
		});
	});
	$effect(() => {
		filteredItems = filterAssignee
			? foundItems.filter((items) => items.assignee === filterAssignee)
			: foundItems;
	});
</script>

<div class="w-full space-y-4">
	<!-- Search & Filter Bar -->
	<div class="flex gap-4 items-center">
		<SearchBar bind:search={searchQuery} />

		<div class="flex flex-row items-center gap-4">
			<form action="?/new" method="post" use:enhance>
				<button class="p-1 text-brick-600 hover:text-brick-500 transition-colors">
					<PlusCircle class="size-8" />
				</button>
			</form>
			<select
				id="assignee"
				name="assignee"
				bind:value={filterAssignee}
				class="w-full rounded-lg border-milk-200 shadow-sm focus:border-brick-500 focus:ring-brick-500"
			>
				<option value="">All Students</option>
				{#each students as student}
					<option value={student.id}>
						{student.name}
					</option>
				{/each}
			</select>
		</div>
	</div>

	<!-- Table -->
	<div class="overflow-x-auto rounded-lg border">
		<table class="w-full">
			<thead class="bg-milk-50">
				<tr>
					{#each config.columns as column}
						<th class="px-6 py-3 text-left text-sm font-semibold">
							{column.label}
						</th>
					{/each}
				</tr>
			</thead>
			<tbody class="divide-y divide-milk-200 bg-white">
				{#each filteredItems as item (item.id)}
					<tr
						onclick={() => goto(`${href}/${item.id}`)}
						class="hover:bg-milk-50 cursor-pointer transition-colors"
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
</div>
