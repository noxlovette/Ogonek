<script lang="ts">
	import type { PageData } from './$types';
	import { LessonCard, H1, Clock, TaskCard } from '$lib/components';
	import { fly } from 'svelte/transition';
	import type { Task, Lesson } from '$lib/types';

	let { data }: { data: PageData } = $props();
	let { tasks, lessons }: { tasks: Task[]; lessons: Lesson[] } = data;

	let pending = tasks.filter((task) => !task.completed);

	$effect(() => {
		pending.sort((a, b) => new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime());
		lessons.sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime());
	});
</script>

<div class="space-y-4">
	<div class="flex justify-between items-center">
		<H1>Dashboard</H1>
		<Clock />
	</div>


	<section class="space-y-4">
		<h2 class="text-xl font-semibold text-milk-800">Recent Tasks</h2>
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
			{#each pending.slice(0, 6) as task (task.id)}
				<div transition:fly={{ y: 20, duration: 300 }}>
					<TaskCard {task} />
				</div>
			{/each}
		</div>
	</section>


	<section class="space-y-4">
		<h2 class="text-xl font-semibold text-milk-800">Latest Lessons</h2>
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
			{#each lessons.slice(0, 6) as lesson (lesson.id)}
				<div transition:fly={{ y: 20, duration: 300 }}>
					<LessonCard {lesson} />
				</div>
			{/each}
		</div>
	</section>
</div>

<svelte:head>
	<title>Dashboard | Firelight</title>
</svelte:head>