<script lang="ts">
	import type { PageData } from './$types';
	import StudentCard from '$lib/components/cards/StudentCard.svelte';
	import Header from '$lib/components/typography/Header.svelte';
	import Table from '$lib/components/UI/Table.svelte';
	import type { Student, TableConfig } from '$lib/types';
	import {formatDateTime} from '$lib/utils';

	let { data }: { data: PageData } = $props();

	const { students } = data;

	console.log(students);

	const studentConfig: TableConfig<Student> = {
		columns: [
			{ key: 'name', label: 'Name' },
			{ key: 'username', label: 'Username' },
			{ key: 'email', label: 'Email' },
			{ key: 'markdown', label: 'Markdown' },
		]
	};

	let href = '/t/students/s';
</script>

<Table config={studentConfig} {href} items={students} {students}/>
