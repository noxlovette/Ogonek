<script lang="ts">
	import type { PageData } from './$types';
	import StudentCard from '$lib/components/cards/StudentCard.svelte';
	import Header from '$lib/components/typography/Header.svelte';
	import Table from '$lib/components/UI/Table.svelte';
	import type { Student, TableConfig } from '$lib/types';
	import {formatDateTime} from '$lib/utils';
	import { enhance } from '$app/forms';
	import {notification} from '$lib/stores';
	import type { SubmitFunction } from '@sveltejs/kit';

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

	interface Result {
		type: 'success' | 'error';
		data: string;
	}

	let href = '/t/students/s';
</script>

<Table config={studentConfig} {href} items={students} {students}/>
<form
	method="POST"
	use:enhance={({ formElement, formData, action, cancel, submitter }) => {
		// `formElement` is this `<form>` element
		// `formData` is its `FormData` object that's about to be submitted
		// `action` is the URL to which the form is posted
		// calling `cancel()` will prevent the submission
		// `submitter` is the `HTMLElement` that caused the form to be submitted

		return async ({ result, update }) => {
			if (result.type === 'success') {
			console.log(result)
            const link: string = result.data.link;
            try {
                await navigator.clipboard.writeText(link);
                notification.set({ message: 'Link copied to clipboard!', type: 'success' });
            } catch (err) {
                notification.set({ message: 'Failed to copy link', type: 'error' });
            }
        } else {
            notification.set({ 
                message: 'Failed to generate link', 
                type: 'error' 
            });
        }
		};
	}}
>

    <button type="submit">
        Generate & Copy Link
    </button>
</form>