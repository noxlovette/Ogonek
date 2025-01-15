<script lang="ts">
	import type { PageData } from './$types';
	import StudentCard from '$lib/components/cards/StudentCard.svelte';
	import Header from '$lib/components/typography/Header.svelte';

	let { data }: { data: PageData } = $props();

	const { students } = data;
</script>

<Header>Students</Header>
{#if students.length === 0}
	<div>No students</div>
{:else}
	<div class="grid grid-cols-2 gap-4">
		{#each students as student}
			<StudentCard {student} />
		{/each}
	</div>
{/if}
