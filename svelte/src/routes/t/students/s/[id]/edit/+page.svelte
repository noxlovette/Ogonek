<script lang="ts">
	import { enhance } from '$app/forms';
	import Editor from '$lib/components/Editor.svelte';
	import Header from '$lib/components/typography/Header.svelte';
	import type { Student } from '$lib/types';
	import type { PageData } from './$types';
	import DOMPurify from 'isomorphic-dompurify';
	let { data }: { data: PageData } = $props();
	let { student } : {student:Student} = data;
	let isSubmitting = $state(false);
	let markdown = $state(student.markdown);

</script>

<form
	method="POST"
	action="?/update"
	class="space-y-4 mb-4"
	use:enhance={() => {

		isSubmitting = true;
		return async ({ result, update }) => {
	
			isSubmitting = false;
			update();
		};
	}}
>
	<div class="flex items-baseline space-x-4">
		<Header>My Notes on {student.name}</Header>
		<a
			href="."
			class="px-4 py-2 text-milk-700 bg-milk-100 rounded-lg hover:bg-milk-200 transition-colors"
		>
			Cancel
		</a>
		<button
			type="submit"
			disabled={isSubmitting}
			class="px-4 py-2 bg-brick-600 text-brick-50 rounded-lg hover:bg-brick-700 focus:outline-none focus:ring-2 focus:ring-brick-500 focus:ring-offset-2 disabled:opacity-50 transition-colors"
		>
			{isSubmitting ? 'Saving...' : 'Save Changes'}
		</button>
		<button
			type="submit"
			disabled={isSubmitting}
			class="px-4 py-2 bg-red-600 text-brick-50 rounded-lg hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-brick-500 focus:ring-offset-2 disabled:opacity-50 transition-colors"
			formaction="?/delete"
		>
			Delete
		</button>
	</div>

	<input type="hidden" name="id" value={student.id} />
	<input type="hidden" name="markdown" value={markdown} />
	
</form>
<Editor bind:markdownContent={markdown} />
