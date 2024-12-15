<script lang='ts'>
    import { getContext } from 'svelte';
    import LessonCardBig from '$lib/components/LessonCardBig.svelte';
    import TaskBox from '$lib/components/TaskBox.svelte';
	import { Squirrel } from 'lucide-svelte';

    const lessons: App.Lesson[] = getContext('lessons');
    const tasks: App.Task[] = getContext('tasks');

    const bookmarked = lessons.filter(lesson => lesson.bookmarked);
</script>


{#if bookmarked.length === 0}
    <Squirrel class='size-32' />
        <p class="text-lg mt-2">No bookmarked lessons found</p>
        <a href="/u/lessons" class="text-sm font-sans p-1 hover:text-sand-800">View All Lessons</a>

    {:else}
    <h1 class='text-4xl my-4'>Bookmarked Lessons</h1>
<div class='grid grid-cols-2 my-4 gap-4'>
    
    {#each bookmarked as lesson}
        <LessonCardBig {lesson} />
    {/each}
</div>
{/if}