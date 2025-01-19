<script lang="ts">
	import type { Lesson } from '$lib/types';
	import { formatDateTime, parseMarkdown } from '$lib/utils';
	import { user } from '$lib/stores';
	import CardClickable from './CardClickable.svelte';
	import { H2 } from '../typography';
	import { onMount } from 'svelte';
	interface Props {
		lesson: Lesson;
	}

	onMount(async () => {
		rendered = await parseMarkdown(lesson.markdown);
	});

	let { lesson }: Props = $props();
	let rendered = $state(lesson.markdown);
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

		<p class=" text-milk-600 text-sm/relaxed lg:text-base/relaxed">
			{@html rendered}
		</p>
	</div>
</CardClickable>
