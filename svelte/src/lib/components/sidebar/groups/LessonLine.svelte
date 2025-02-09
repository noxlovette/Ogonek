<script lang="ts">
	import { formatDateTime } from '$lib/utils';
	import type { Lesson } from '$lib/types';
	import { user } from '$lib/stores';
	interface Props {
		lesson: Lesson;
	}

	let { lesson }: Props = $props();

	const formattedDate = formatDateTime(lesson.manualDate || lesson.createdAt);
	let href = $user.role === 'teacher' ? `/t/lessons/l/${lesson.id}` : `/s/lessons/l/${lesson.id}`;
</script>

<a
	{href}
	class="px-2 py-1 flex flex-col transition-all duration-300 rounded-lg hover:text-cacao-400 hover:translate-x-1"
>
	<h3>
		{formattedDate}
	</h3>
	<p class="text-base">
		{lesson.topic}
	</p>
</a>
