<script lang="ts">
	import type { PageData } from './$types';
	import StudentCard from '$lib/components/cards/StudentCard.svelte';
	import Header from '$lib/components/typography/Header.svelte';
	import Table from '$lib/components/UI/Table.svelte';
	import type { Student, TableConfig } from '$lib/types';
	import {formatDateTime} from '$lib/utils';
	import { enhance } from '$app/forms';
	import {notification} from '$lib/stores';

	let { data }: { data: PageData } = $props();

	const { students } = data;

	const studentConfig: TableConfig<Student> = {
		columns: [
			{ key: 'name', label: 'Name' },
			{ key: 'username', label: 'Username' },
			{ key: 'email', label: 'Email' },
			{ key: 'markdown', label: 'Markdown' },
		]
	};


const handle = async ({ result, update }: { result: any; update: () => void }) => {

	console.log(result)

if (result.type === 'success') {
	const { data } = result.data;
	notification.set({ message: 'Link Generated', type: 'success' });
	console.log(data);
} else {
	if (result.data) {
		notification.set({
			message: result.data.message || 'Generation failed',
			type: 'error'
		});
	} else {
		notification.set({ message: 'Login failed', type: 'error' });
	}
}
update();
};

	let href = '/t/students/s';
</script>

<Table config={studentConfig} {href} items={students} {students}/>

<form method="post" class="bg-black" use:enhance={() => handle}>
	<button type="submit" onclick={() => console.log('HELLO')}>
		HELLO
	</button>
</form>
