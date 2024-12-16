<script lang="ts">
	import LessonCardBig from "$lib/components/LessonCardBig.svelte";
    import { getContext } from "svelte";

    const lessons:App.Lesson[] = getContext("lessons");
</script>
<h1 class="text-4xl">
    My Lessons
</h1>
<div class="grid grid-cols-2 my-4 gap-4">
{#each lessons as lesson}
<LessonCardBig lesson={lesson} />
{/each}

</div>

<a href="/u/lessons/bookmarked" class="text-sm font-sans p-1 hover:text-sand-800">View Bookmarked</a>