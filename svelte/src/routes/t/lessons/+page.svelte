<script lang="ts">
	import type { PageData } from './$types';
	import type { Lesson, TableConfig } from '$lib/types';
	import { user } from '$lib/stores';
	import Header from '$lib/components/typography/Header.svelte';
	import { formatDateTime } from '$lib/utils';
	import Table from '$lib/components/UI/Table.svelte';
	import { ButtonSubmit } from '$lib/components/UI/buttons';
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

<Header>Lessons</Header>
<Table items={lessons} config={lessonConfig} {href} {students} />

<form action="?/new" method="post" use:enhance>
	{#if lessons.length === 0}
		<ButtonSubmit buttonName="Add your first one!" />
	{/if}
</form>
