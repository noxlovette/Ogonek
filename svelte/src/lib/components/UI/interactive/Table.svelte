<script lang="ts" generics="T extends BaseTableItem">
	import { enhance } from '$app/forms';
	import { PlusCircle } from 'lucide-svelte';
	import { Search } from 'lucide-svelte';
	import { goto } from '$app/navigation';
	import { fade } from 'svelte/transition';
	import { notification } from '$lib/stores';
	import type { Student, BaseTableItem, TableConfig } from '$lib/types';

	interface Props<T extends BaseTableItem> {
		items: T[];
		config: TableConfig<T>;
		href: string;
		students: Student[];
	}

	let { items, config, href, students = [] }: Props<T> = $props();

	let query = $state('');
	let filteredItems = $state(items);
	let foundItems = $state(items);
	let filterAssignee = $state('');
	let isSubmitting = $state(false);

	$effect(() => {
		const lowercaseQuery = query.toLowerCase();

		foundItems = items.filter((item) => {
			// Search through all configured columns
			return (
				config.columns
					.filter((column) => column.searchable !== false)
					.some((column) => {
						const value = item[column.key];
						// Safely handle non-string values using the formatter or convert to string
						const searchableValue = column.formatter
							? String(column.formatter(value)).toLowerCase() // Ensure formatter output is a string
							: String(value).toLowerCase();

						return searchableValue.includes(lowercaseQuery);
					}) ||
				// Always include assignee in search (since it's part of BaseTableItem)
				(item.assignee && item.assignee.toLowerCase().includes(lowercaseQuery))
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
		{#if items.length !== 0}
			<div class="hidden md:relative flex-1">
				<Search class="absolute left-3 top-1/2 -translate-y-1/2 text-milk-400 " />
				<input
					type="text"
					bind:value={query}
					placeholder="Search..."
					class="w-full pl-10 pr-4 py-2 border rounded-full focus:ring-2 bg-brick-50 border-milk-200 focus:ring-brick-500 focus:border-transparent
			focus:placeholder:text-brick-400/70
			placeholder:text-milk-500
			"
				/>
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
					<button class="p-1 text-brick-600 hover:text-brick-500 transition-colors">
						<PlusCircle class="size-8" />
					</button>
				</form>
				{#if students.length > 0}
					<select
						id="assignee"
						name="assignee"
						bind:value={filterAssignee}
						class="w-full text-xs md:text-base rounded-lg border-milk-200 shadow-sm focus:border-brick-500 focus:ring-brick-500"
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
		<div class="overflow-x-auto rounded-lg border shadow-md">
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
	{/if}
</div>
