<script lang="ts">
	import type { PageData } from './$types';

	import type { Lesson } from '$lib/types';
	import { goto } from '$app/navigation';
	import { user } from '$lib/stores';
	import Header from '$lib/components/typography/Header.svelte';
	let { data }: { data: PageData } = $props();

	import { fade, slide } from 'svelte/transition';
	import { formatDateTime } from '$lib/utils';
	import SearchBar from '$lib/components/UI/SearchBar.svelte';

	let { lessons, students } = data;
	let searchQuery = $state('');
	let filteredLessons = $state(lessons);
	let foundLessons = $state(lessons);
	let filterAssignee = $state('');

	$effect(() => {
		const lowercaseQuery = searchQuery.toLowerCase();
		foundLessons = lessons.filter(
			(lesson: Lesson) =>
				lesson.title.toLowerCase().includes(lowercaseQuery) ||
				lesson.topic.toLowerCase().includes(lowercaseQuery) ||
				lesson.assigneeName.toLowerCase().includes(lowercaseQuery)
		);
	});

	// Separate effect for assignee filtering
	$effect(() => {
		filteredLessons = filterAssignee
			? foundLessons.filter((lesson: Lesson) => lesson.assignee === filterAssignee)
			: foundLessons;
	});
</script>

<Header>Lessons</Header>
<div class="w-full space-y-4">
	<!-- Search & Filter Bar -->
	<div class="flex gap-4 items-center">
		<SearchBar bind:search={searchQuery} />

		<div class="">
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
					<th class="px-6 py-3 text-left text-sm font-semibold">Title</th>
					<th class="px-6 py-3 text-left text-sm font-semibold">Topic</th>
					<th class="px-6 py-3 text-left text-sm font-semibold">Assignee</th>
					<th class="px-6 py-3 text-left text-sm font-semibold">Created</th>
				</tr>
			</thead>
			<tbody class="divide-y divide-milk-200 bg-white">
				{#each filteredLessons as lesson (lesson.id)}
					<tr
						onclick={() => goto(`lessons/l/${lesson.id}`)}
						class="hover:bg-milk-50 cursor-pointer transition-colors"
						in:fade={{ duration: 200 }}
					>
						<td class="px-6 py-4 text-sm">{lesson.title}</td>
						<td class="px-6 py-4 text-sm text-milk-500">{lesson.topic}</td>
						<td class="px-6 py-4 text-sm text-milk-500">
							{#if lesson.assigneeName === $user.username}
								Not Assigned
							{:else}
								{lesson.assigneeName}
							{/if}
						</td>
						<td class="px-6 py-4 text-sm text-milk-500">
							{formatDateTime(lesson.createdAt)}
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>
