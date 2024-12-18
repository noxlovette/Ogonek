<script lang='ts'>
    
    import Header from '$lib/components/fonts/Header.svelte';
import LessonCardBig from '$lib/components/LessonCardBig.svelte';
    
	import { Squirrel } from 'lucide-svelte';

    export let data:PageServerData;
    let lessons: App.Lesson[] = data.lessons;
    let bookmarked = lessons.filter(lesson => lesson.bookmarked);

</script>


{#if bookmarked.length === 0}
    <Squirrel class='size-32' />
        <p class="text-lg mt-2">No bookmarked lessons found</p>
        
        {:else}

        <Header>
            Bookmarked
        </Header>
        <div class='grid grid-cols-2 my-4 gap-4'>
            {#each bookmarked as lesson}
            <LessonCardBig {lesson} />
            {/each}
        </div>
        {/if}
        <a href="/u/lessons" class="text-sm font-sans hover:text-sand-800">View All Lessons</a>