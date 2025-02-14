<script lang="ts">
  import LessonCard from "$lib/components/sidebar/groups/LessonLine.svelte";
  import H2 from "$lib/components/typography/H2.svelte";
  import Group from "./Group.svelte";
  import { lessonStore } from "$lib/stores";
  import SmallLink from "$lib/components/typography/SmallLink.svelte";
</script>

<Group>
  <H2>Recent Lessons</H2>
  <ul class="">
    {#if $lessonStore.length === 0}
      <p>No lessons found</p>
    {:else}
      {#each $lessonStore.slice(0, 3) as lesson}
        <LessonCard {lesson} />
      {/each}
    {/if}
  </ul>
  <SmallLink href="/s/lessons">View All</SmallLink>
</Group>
