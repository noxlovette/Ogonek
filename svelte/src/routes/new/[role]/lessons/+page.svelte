<script lang="ts">
	import type { PageData } from './$types';
	import { H1, Table, ButtonSubmit, H2, LessonCard } from '$lib/components';
	import { enhance } from '$app/forms';
	import {page} from "$app/state"
	import type { TableConfig, Lesson } from '$lib/types';
import { formatDateTime } from '$lib/utils';
import {user} from "$lib/stores"

	let { data }: { data: PageData } = $props();
	let { lessons, students } = data;

	let href = $user.role === 'teacher' ? `/t/lessons/l` : `/s/lessons/l`;

	let role = $derived(page.params.role);


const lessonConfig: TableConfig<Lesson> = {
	columns: [
		{ key: 'title', label: 'Title' },
		{ key: 'topic', label: 'Topic' },
		{
			key: 'assigneeName',
			label: 'Assignee',
			formatter: (value: string) => (value === $user.name ? 'Not Assigned' : value)
		},
		{ key: 'createdAt', label: 'Created', formatter: (value: string) => formatDateTime(value) }
	]
};

</script>

<H1>Lessons</H1>

{#if role === "t"}
<Table items={lessons} config={lessonConfig} {href} {students} />

<form action="?/new" method="post" use:enhance>
	{#if lessons.length === 0}
		<ButtonSubmit buttonName="Add your first one!" />
	{/if}
</form>
{:else}
<section class="space-y-4">
	<H2>Recent Lessons</H2>
	<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
		{#each lessons as lesson}
			<LessonCard {lesson} />
		{/each}
	</div>
</section>

{/if}

<svelte:head>
	<title>Lessons</title>
</svelte:head>