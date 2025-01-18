<script lang="ts">
	import LessonCard from '$lib/components/sidebar/groups/LessonLine.svelte';
	import type { Lesson } from '$lib/types';
	import H2 from '$lib/components/typography/H2.svelte';

	import { getContext } from 'svelte';
	import Group from './Group.svelte';
	import SmallLink from '$lib/components/typography/SmallLink.svelte';

	let lessons: Lesson[] = $state(getContext('lessons') || []);
</script>

<Group>
	<H2>Recent Lessons</H2>
	<ul class="">
		{#if lessons.length === 0}
			<p>No lessons found</p>
		{:else}
			{#each lessons as lesson}
				<LessonCard {lesson} />
			{/each}
		{/if}
	</ul>
	<SmallLink href="/s/lessons">View All</SmallLink>
</Group>
