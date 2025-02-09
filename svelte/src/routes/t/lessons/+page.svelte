<script lang="ts">
	import type { PageData } from './$types';
	import type { Lesson, TableConfig } from '$lib/types';
	import { H1, Table, ButtonSubmit } from '$lib/components';
	import { user } from '$lib/stores';
	import { formatDateTime } from '$lib/utils';
	import { enhance } from '$app/forms';

	let { data }: { data: PageData } = $props();
	let { lessons, students } = data;

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

	let href = $user.role === 'teacher' ? `/t/lessons/l` : `/s/lessons/l`;
</script>

<H1>Lessons</H1>
<Table items={lessons} config={lessonConfig} {href} {students} />

<form action="?/new" method="post" use:enhance>
	{#if lessons.length === 0}
		<ButtonSubmit buttonName="Add your first one!" />
	{/if}
</form>

<svelte:head>
	<title>Lessons</title>
</svelte:head>