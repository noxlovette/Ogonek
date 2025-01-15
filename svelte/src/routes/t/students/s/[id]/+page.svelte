<script lang="ts">
	import Header from '$lib/components/typography/Header.svelte';
	import { user } from '$lib/stores';
	import type { Student } from '$lib/types';
	import { formatDateTime } from '$lib/utils';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	const { student, rendered } : {student: Student, rendered: String} = data;
</script>

<svelte:head>
	<title>{student.name}</title>
</svelte:head>
	<div class="flex items-baseline space-x-4">
		<Header>{student.name}</Header>
		<a
			href="/t/students/s/{student.id}/edit"
			class="px-4 py-2 bg-brick-600 text-brick-50 rounded-lg hover:bg-brick-700 focus:outline-none focus:ring-2 focus:ring-brick-500 focus:ring-offset-2 disabled:opacity-50 transition-colors"
			>Edit</a
		>
	</div>
	<div class="flex space-x-4">
		<div class="space-y-2">
			<p class="block font-medium text-milk-700">Email</p>
			<h3 class="min-w-48">
				{student.email}
			</h3>
		</div>

	</div>
	<h3 class="text-2xl font-bold">Notes</h3>
    {#if student.markdown}
	<div class="markdown ring-2 ring-milk-200 p-4 rounded-lg">{@html rendered}</div>
    {:else}
    <p>No notes yet</p>
    {/if}
