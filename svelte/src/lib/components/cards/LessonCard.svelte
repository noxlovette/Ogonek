<script lang="ts">
	import type { Lesson } from '$lib/types';
	import { formatDateTime } from '$lib/utils';
	import { user } from '$lib/stores';
	import CardClickable from './CardClickable.svelte';
	import { H2 } from '../typography';
	interface Props {
		lesson: Lesson;
	}

	let { lesson }: Props = $props();

	const formattedDate = formatDateTime(lesson.manualDate || lesson.createdAt);
	let href = $user.role === 'teacher' ? `/t/lessons/l/${lesson.id}` : `/s/lessons/l/${lesson.id}`;
</script>

<CardClickable {href}>
	<H2>
		{formattedDate}
	</H2>

	<div class="flex flex-col">
		<h2
			class="text-lg font-semibold
			"
		>
			{lesson.topic}
		</h2>

		<p class="prose text-milk-600 text-sm/relaxed lg:text-base/relaxed">
			{lesson.markdown}
		</p>
	</div>
</CardClickable>
