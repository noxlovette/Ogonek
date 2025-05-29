<script lang="ts">
  import LessonLine from "$lib/components/sidebar/groups/LessonLine.svelte";
  import H2 from "$lib/components/typography/H2.svelte";
  import Group from "./Group.svelte";
  import { lessonStore } from "$lib/stores";
  import { sidebar } from "$lib/stores";
  import UniButton from "$lib/components/UI/UniButton.svelte";
  import { Eye } from "lucide-svelte";
</script>

<Group>
  <H2>Recent Lessons</H2>

  <div class={$sidebar ? "flex-row space-y-2 py-2" : "flex-col"}>
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
