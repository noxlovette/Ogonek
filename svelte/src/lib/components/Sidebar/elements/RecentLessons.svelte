<script lang="ts">
	import LessonCard from '$lib/components/lesson/LessonCard.svelte';
	import type { Lesson } from '$lib/types';

	import { getContext } from 'svelte';
	import Group from './Group.svelte';
	import SmallLink from '$lib/components/typography/SmallLink.svelte';

	let lessons: Lesson[] = $state(getContext('lessons') || []);
</script>

<Group>
	<h2 class=" lg:text-lg xl:text-xl">Recent</h2>
	<ul class="">
		{#if lessons.length === 0}
			<p>No lessons found</p>
		{:else}
			{#each lessons as lesson}
				<LessonCard {lesson} />
			{/each}
		{/if}
	</ul>
	<SmallLink href="/u/lessons">View All</SmallLink>
</Group>
