<script lang="ts">
    export let lesson: App.Lesson;

    const date = new Date(lesson.manual_date||lesson.created_at);

const formattedDate = date.toLocaleDateString('en-GB', {
    month: 'short',
    day: 'numeric',
    year: 'numeric'
});
</script>


<a href="/u/lessons/{lesson.id}" class="px-2 py-1 flex flex-col">
    <h3>
        {formattedDate}
    </h3>
    <p class="text-base">
        {lesson.topic}
    </p>
</a>

<style>
    a:hover {
        @apply bg-sand-900/20 transition-colors duration-300 rounded-lg text-sand-100;
    }
</style>