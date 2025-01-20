<script lang="ts">
	import { enhance } from '$app/forms';
	import { ButtonSubmit, Editor, H1 } from '$lib/components';
	import type { Student } from '$lib/types';
	import { Send } from 'lucide-svelte';
	import type { PageData } from './$types';
	import ButtonDelete from '$lib/components/UI/buttons/ButtonDelete.svelte';
	let { data }: { data: PageData } = $props();
	let { student }: { student: Student } = data;
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
			if (result.type === 'redirect') {
				notification.set({ message: 'Changes saved', type: 'success' });
				update();
			} else {
				notification.set({
					message: 'Failed to save changes',
					type: 'error'
				});
			}
		};
	}}
>
	<div class="flex items-baseline justify-between">
		<H1>{student.name}</H1>
		<div class="flex items-center space-x-3">
			<div class="relative">
				<input
					type="text"
					name="telegramId"
					value={student.telegramId}
					placeholder="@username"
					class="pl-10 pr-4 py-2 border border-milk-300 rounded-lg focus:ring-2 focus:ring-brick-500 focus:border-transparent"
				/>
				<span class="absolute left-3 top-2.5 text-milk-400">
					<Send></Send>
				</span>
			</div>
			<a
				href="."
				class="px-4 py-2 text-milk-700 bg-milk-100 rounded-lg hover:bg-milk-200 transition-colors"
			>
				Cancel
			</a>
			<ButtonSubmit bind:isSubmitting></ButtonSubmit>
			<ButtonDelete bind:isSubmitting></ButtonDelete>
		</div>

		<input type="hidden" name="id" value={student.id} />
		<input type="hidden" name="markdown" value={markdown} />
	</div>
</form>
<Editor bind:markdownContent={markdown} />
