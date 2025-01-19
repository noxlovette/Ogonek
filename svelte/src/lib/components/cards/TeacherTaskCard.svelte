<script lang="ts">
	import { formatDateTime } from '$lib/utils';
	import type { Task } from '$lib/types';
	import Clickable from './CardClickable.svelte';
	interface Props {
		task: Task;
	}

	let { task }: Props = $props();

	let overdue = false;
	let completed = $state(task.completed);
	const formattedDate = formatDateTime(task.dueDate);
</script>

<Clickable href="/t/tasks/t/{task.id}">
	<div
		id="task-header"
		class="inline-flex space-x-8 text-lg md:text-xl lg:text-2xl xl:text-3xl justify-between items-baseline"
	>
		<h2 class="flex">{task.title}</h2>
	</div>

	<p class="text-sm lg:text-base opacity-30">{@html task.markdown}</p>

	<div id="task-footer" class="items-center mt-auto text-sm inline-flex space-x-1 justify-between">
		<p class:overdue class="opacity-60">Due {formattedDate}</p>
	</div>
</Clickable>

<style lang="postcss">
	.overdue {
		@apply text-brick-600 font-bold underline;
	}

	button.overdue {
		@apply text-inherit;
	}

	.completed {
		@apply opacity-50;
	}
</style>
