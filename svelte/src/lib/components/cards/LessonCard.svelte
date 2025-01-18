<script lang="ts">
	import type { Lesson } from '$lib/types';
	import { formatDateTime } from '$lib/utils';
	import { user } from '$lib/stores';
	import CardClickable from './CardClickable.svelte';
	interface Props {
		lesson: Lesson;
	}

	let { lesson }: Props = $props();

	const formattedDate = formatDateTime(lesson.manualDate || lesson.createdAt);
	let href = $user.role === 'teacher' ? `/t/lessons/l/${lesson.id}` : `/s/lessons/l/${lesson.id}`;
</script>

<CardClickable {href}>
	<div class="flex flex-col">
		<h3 class="text-3xl">
			{formattedDate}
		</h3>

		<div class="flex flex-col space-y-2">
			<h2 class="text">
				{lesson.topic}
			</h2>
			<p class="opacity-30">
				{lesson.markdown}
			</p>
		</div>
	</div>
</CardClickable>
