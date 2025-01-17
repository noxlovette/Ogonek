<script lang="ts">
	import { enhance } from '$app/forms';
	import Editor from '$lib/components/Editor.svelte';
	import Header from '$lib/components/typography/Header.svelte';
	import { ButtonSubmit, ButtonDelete } from '$lib/components/UI/buttons/index';
	import type { PageData } from './$types';
	let { data }: { data: PageData } = $props();
	let { task, students } = data;
	let isSubmitting = $state(false);
	let markdown = $state(task.markdown);
</script>

<form method="POST" action="?/update" class="space-y-4 mb-4" use:enhance>
	<div class="flex items-baseline space-x-4">
		<Header>Edit Task</Header>
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
	<div class="flex space-x-4">
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
				name="assignee"
				value={task.assignee}
				class="w-full rounded-lg border-milk-200 shadow-sm focus:border-brick-500 focus:ring-brick-500"
			>
				<option value="">Select an assignee</option>
				{#each students as student}
					<option value={student.id} selected={student.id === task.assignee}>
						{student.name}
					</option>
				{/each}
			</select>
		</div>
	</div>
</form>
<Editor bind:markdownContent={markdown} />
