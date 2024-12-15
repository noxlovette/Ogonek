<script lang="ts">
	
	import LessonCard from "./LessonCard.svelte";


</script>

<div class="p-4 border-2 rounded-lg border-sand-900/20">
<h2 class="text-xl">
    Lessons
</h2>
<ul class="">
<LessonCard />
<LessonCard />
<LessonCard />
</ul>
<a href="/u/lessons" class="text-sand-400 text-sm font-sans p-1">View All Lessons

</a>
</div>

<style>
    a:hover {
        @apply bg-sand-900/20 transition-colors duration-300 rounded-lg text-sand-100;
    }
</style>