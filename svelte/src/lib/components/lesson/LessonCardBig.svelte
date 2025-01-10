<script lang="ts">
	import { formatDate } from '$lib/utils';
	interface Props {
		lesson: App.Lesson;
	}

	let { lesson }: Props = $props();

	const formattedDate = formatDate(lesson.manual_date || lesson.created_at);
</script>

<a
	href="/u/lessons/l/{lesson.id}"
	class="flex flex-col hover:bg-brick-900/60 hover:text-brick-100 transition-colors shadow border border-brick-900/10 rounded-lg max-h-[150px] overflow-clip py-2"
>
	<div class="flex flex-col py-3 px-5">
		<h3 class="text-3xl">
			{formattedDate}
		</h3>

		<div class="flex flex-col space-y-2">
			<h2 class="text">
				{lesson.topic}
			</h2>
			<p class="opacity-30">
				{lesson.content}
			</p>
		</div>
	</div>
</a>
