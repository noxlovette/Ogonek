<script lang="ts">
	import Header from "$lib/components/fonts/Header.svelte";
import LessonCardBig from "$lib/components/LessonCardBig.svelte";
    import { getContext } from "svelte";

    const lessons:App.Lesson[] = getContext("lessons");
</script>
<Header>
    Lessons
</Header>
<div class="grid grid-cols-2 my-4 gap-4">
{#each lessons as lesson}
<LessonCardBig lesson={lesson} />
{/each}

</div>

<a href="/u/lessons/bookmarked" class="text-sm font-sans p-1 hover:text-sand-800">View Bookmarked</a>