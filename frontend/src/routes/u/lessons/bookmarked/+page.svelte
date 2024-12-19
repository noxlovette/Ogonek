<script lang="ts">
	import Header from '$lib/components/fonts/Header.svelte';
	import LessonCardBig from '$lib/components/LessonCardBig.svelte';

	import { Squirrel } from 'lucide-svelte';
	import { language, translations } from '$lib/stores';

	export let data: PageServerData;
	let lessons: App.Lesson[] = data.lessons;
	let bookmarked = lessons.filter((lesson) => lesson.bookmarked);
</script>

{#if bookmarked.length === 0}
	<Squirrel class="size-32" />
	<p class="text-lg mt-2">
		{$translations.no_bookmarks[$language]}
	</p>
{:else}
	<Header>
		{$translations.bookmarked[$language]}
	</Header>
	<div class="grid grid-cols-2 my-4 gap-4">
		{#each bookmarked as lesson}
			<LessonCardBig {lesson} />
		{/each}
	</div>
{/if}
<a href="/u/lessons" class="text-sm font-sans hover:text-sand-800">
	{$translations.view_all[$language]}
</a>
