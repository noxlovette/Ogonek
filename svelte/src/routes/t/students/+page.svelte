<script lang="ts">
	import type { PageData } from './$types';
	import { Table, ButtonSubmit, H1 } from '$lib/components';
	import type { Student, TableConfig } from '$lib/types';
	import { enhance } from '$app/forms';
	import { notification } from '$lib/stores';

	let { data }: { data: PageData } = $props();

	const { students } = data;

	const studentConfig: TableConfig<Student> = {
		columns: [
			{ key: 'name', label: 'Name' },
			{ key: 'username', label: 'Username' },
			{ key: 'email', label: 'Email' },
			{ key: 'markdown', label: 'Markdown' }
		]
	};

	interface Result {
		type: 'success' | 'error';
		data: string;
	}

	let href = '/t/students/st';

	let isSubmitting = $state(false);
</script>

<H1>Students</H1>
<Table config={studentConfig} {href} items={students} {students} />
<form
	method="POST"
	use:enhance={({ formElement, formData, action, cancel, submitter }) => {
		isSubmitting = true;
		// `formElement` is this `<form>` element
		// `formData` is its `FormData` object that's about to be submitted
		// `action` is the URL to which the form is posted
		// calling `cancel()` will prevent the submission
		// `submitter` is the `HTMLElement` that caused the form to be submitted

		return async ({ result }) => {
			isSubmitting = false;
			if (result.type === 'success') {
				const link = String(result.data?.link);
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
	<ButtonSubmit bind:isSubmitting buttonName="Invite Students" />
</form>
