<script lang="ts">
	import type { PageData } from './$types';
	import type { Student } from '$lib/types';

	import { getContext } from 'svelte';

	let { data }: { data: PageData } = $props();

	const students: Student[] = getContext('students');
</script>

{#if students.length === 0}
	<div>No students</div>
{:else}
	<div>Students</div>
{/if}

{#each students as student}
	<div>{student.name}</div>
{/each}
