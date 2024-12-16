<script lang="ts">
	export let lesson: App.Lesson;

	const date = new Date(lesson.manual_date || lesson.created_at);

	const formattedDate = date.toLocaleDateString('en-GB', {
		month: 'short',
		day: 'numeric',
		year: 'numeric'
	});
</script>

<a href="/u/lessons/l/{lesson.id}" class="flex flex-col hover:bg-sand-900/60 hover:text-sand-100 transition-colors shadow border border-sand-900/10 rounded-lg max-h-[150px] overflow-clip py-2">
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
