<script lang="ts">
	import { formatDateTime } from '$lib/utils';
	import type { Task } from '$lib/types';
	import Clickable from './CardClickable.svelte';
	import { user } from '$lib/stores';
	import { H2 } from '../typography';

	interface Props {
		task: Task;
	}

	let { task }: Props = $props();
	let overdue = false;
	const formattedDate = formatDateTime(task.dueDate);
	let href = $user.role === 'teacher' ? `/t/tasks/t/${task.id}` : '/s/tasks';
</script>

<Clickable {href}>
	<H2>
		{task.title}
	</H2>

	<div class="mt-auto pt-4 flex items-center justify-between text-sm/tight">
		<p class:overdue class="text-milk-500 {overdue ? 'text-brick-500' : ''}">
			Due {formattedDate}
		</p>
	</div>

	<p class="prose text-milk-600 text-sm/relaxed lg:text-base/relaxed">
		{@html task.markdown}
	</p>
</Clickable>

<style lang="postcss">
	/* Optional: Add any component-specific styles here */
	:global(.overdue) {
		@apply font-medium text-red-600;
	}
</style>
