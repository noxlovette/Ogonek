<script lang="ts">
	import Header from '$lib/components/typography/Header.svelte';
	import LessonCardBig from '$lib/components/lesson/LessonCardBig.svelte';

	import { Squirrel } from 'lucide-svelte';
	import type { Lesson } from '$lib/types';
	import type { PageServerData } from './$types';

	interface Props {
		data: PageServerData;
	}

	let { data }: Props = $props();
	let lessons: Lesson[] = data.lessons;
	let bookmarked = lessons.filter((lesson) => lesson.bookmarked);
</script>

<svelte:head>
	<title>Bookmarked</title>
</svelte:head>

{#if bookmarked.length === 0}
	<Squirrel class="size-32" />
	<p class="text-lg mt-2">No Bookmarks</p>
{:else}
	<Header>Bookmarked</Header>
	<div class="grid grid-cols-2 my-4 gap-4">
		{#each bookmarked as lesson}
			<LessonCardBig {lesson} />
		{/each}
	</div>
{/if}
<a href="/u/lessons" class="text-sm font-sans hover:text-brick-800"> View All </a>
