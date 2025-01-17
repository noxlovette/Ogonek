<script lang="ts">
	import type { PageData } from './$types';
	import { user } from '$lib/stores';
	import type { Task, TableConfig } from '$lib/types';
	import Header from '$lib/components/typography/Header.svelte';
	import { formatDateTime } from '$lib/utils';
	import Table from '$lib/components/UI/Table.svelte';
	import { ButtonSubmit } from '$lib/components/UI/buttons';
	import {enhance} from '$app/forms';

	let { data }: { data: PageData } = $props();

	const { tasks, students } = data;

	const taskConfig: TableConfig<Task> = {
		columns: [
			{ key: 'title', label: 'Title' },
			{ key: 'markdown', label: 'Markdown' },
			{
				key: 'assigneeName',
				label: 'Assignee',
				formatter: (value: string) => (value === $user.name ? 'Not Assigned' : value)
			},
			{ key: 'createdAt', label: 'Created', formatter: (value: string) => formatDateTime(value) }
		]
	};

	let href = $user.role === 'teacher' ? `/t/tasks/t` : `/s/tasks/t`;
</script>

<Header>Tasks</Header>

<Table items={tasks} config={taskConfig} {href} {students} />

<form action="?/new" method="post" use:enhance>
	{#if tasks.length === 0}
		<ButtonSubmit buttonName="Add your first one!" />
	{/if}
</form>
