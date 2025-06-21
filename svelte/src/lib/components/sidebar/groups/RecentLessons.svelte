<script lang="ts">
  import LessonLine from "$lib/components/sidebar/groups/LessonLine.svelte";

  import { H3 } from "$lib/components/typography";
  import Group from "./Group.svelte";
  import { lessonStore } from "$lib/stores";
  import UniButton from "$lib/components/UI/UniButton.svelte";
  import { Eye } from "lucide-svelte";
</script>

<Group>
  <H3>Recent Lessons</H3>

  <div class="grid gap-4">
    {#if $lessonStore.length === 0}
      <p>No lessons found</p>
    {:else}
      {#each $lessonStore.slice(0, 2) as lesson (lesson.id)}
        <LessonLine {lesson} />
      {/each}
    {/if}

    <UniButton
      size="sm"
      Icon={Eye}
      fullWidth={false}
      href="/s/lessons"
      type="button"
      variant="ghost">View All</UniButton
    >
  </div>
</Group>
