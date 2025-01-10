<script lang="ts">

	import { formatDate } from '$lib/utils';
	interface Props {
		lesson: App.Lesson;
	}

	let { lesson }: Props = $props();

	const formattedDate = formatDate(lesson.manual_date || lesson.created_at);
</script>

<a href="/u/lessons/l/{lesson.id}" class="px-2 py-1 flex flex-col hover:bg-forest-700 transition-colors duration-300 rounded-lg hover:text-forest-100">
	<h3>
		{formattedDate}
	</h3>
	<p class="text-base">
		{lesson.topic}
	</p>
</a>
