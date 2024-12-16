<script lang="ts">
	export let lesson: App.Lesson;

	const date = new Date(lesson.manual_date || lesson.created_at);

	const formattedDate = date.toLocaleDateString('en-GB', {
		month: 'short',
		day: 'numeric',
		year: 'numeric'
	});
</script>

<a
	href="/u/lessons/{lesson.id}"
	class="px-4 py-2 flex flex-col rounded-lg hover:bg-sand-900/60 hover:text-sand-100 transition-colors duration-300 ring ring-sand-900/20"
>
	<h3 class="text-3xl">
		{formattedDate}
	</h3>
	<p class="text-xl">
		{lesson.topic}
	</p>
</a>
