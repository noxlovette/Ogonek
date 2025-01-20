<script lang="ts">
	import { enhance } from '$app/forms';
	import { notification } from '$lib/stores';
	import { Editor, H1, ButtonDelete, ButtonSubmit, Uploader } from '$lib/components';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
	let { task, students } = data;
	let isSubmitting = $state(false);
	let markdown = $state(task.markdown);

	let dueDate = $state(task.dueDate ? new Date(task.dueDate).toISOString().split('T')[0] : '');

	$effect(() => {
		if (task.dueDate) {
			dueDate = new Date(task.dueDate).toISOString().split('T')[0];
		}
	});
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


	<div class="flex items-baseline space-x-4">
		<H1>Edit Task</H1>
		<a
			href="."
			class="px-4 py-2 text-milk-700 bg-milk-100 rounded-lg hover:bg-milk-200 transition-colors"
		>
			Cancel
		</a>
		<ButtonSubmit bind:isSubmitting />
		<ButtonDelete bind:isSubmitting />
	</div>

	<input type="hidden" name="id" value={task.id} />
	<input type="hidden" name="markdown" value={markdown} />
	

	<div class="grid grid-cols-3 gap-4">
		<div class="space-y-2">
			<label for="title" class="block font-medium text-milk-700">Title</label>
			<input
				id="title"
				type="text"
				name="title"
				value={task.title}
				class="w-full rounded-lg border-milk-200 shadow-sm focus:border-brick-500 focus:ring-brick-500"
			/>
		</div>
		<div class="space-y-2">
			<label for="assignee" class="block font-medium text-milk-700">Assignee</label>
			<select
				id="assignee"
				name="student"
				value={task.assignee}
				class="w-full rounded-lg border-milk-200 shadow-sm focus:border-brick-500 focus:ring-brick-500"
			>
				<option value="">Select an assignee</option>
				{#each students as student}
					<option value={JSON.stringify({
						assignee: student.id,
						telegramId: student.telegramId
					  })} selected={student.id === task.assignee}>
						{student.name}
					</option>
				{/each}
			</select>
		</div>
		<div class="space-y-2">
			<label for="dueDate" class="block font-medium text-milk-700">Due Date</label>
			<input
				id="dueDate"
				type="date"
				name="dueDate"
				bind:value={dueDate}
				class="w-full rounded-lg border-milk-200 shadow-sm focus:border-brick-500 focus:ring-brick-500"
			/>
		</div>
		<div class="space-y-2 flex items-center">
			<label class="relative inline-flex items-center cursor-pointer">
				<input type="checkbox" name="completed" checked={task.completed} class="sr-only peer" />
				<div
					class="w-11 h-6 bg-milk-200 peer-focus:outline-none peer-focus:ring-4
					   peer-focus:ring-brick-300 rounded-full peer
					   peer-checked:after:translate-x-full peer-checked:after:border-white
					   after:content-[''] after:absolute after:top-[2px] after:left-[2px]
					   after:bg-white after:border-milk-300 after:border after:rounded-full
					   after:h-5 after:w-5 after:transition-all peer-checked:bg-brick-600"
				></div>
				<span class="ml-3 text-sm font-medium text-milk-700">Completed</span>
			</label>
		</div>
	</div>
</form>

<Editor bind:markdownContent={markdown} />

<Uploader id={task.id} />
