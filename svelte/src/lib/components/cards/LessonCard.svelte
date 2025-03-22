<script lang="ts">
  import type { LessonSmall } from "$lib/types";
  import { parseMarkdown } from "$lib/utils";
  import { formatDate } from "@noxlovette/svarog";
  import CardClickable from "./CardClickable.svelte";
  import { H2 } from "../typography";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  interface Props {
    lesson: LessonSmall;
  }

  onMount(async () => {
    rendered = await parseMarkdown(lesson.markdown);
  });

  let { lesson }: Props = $props();
  let rendered = $state(lesson.markdown);

  const formattedDate = formatDate(lesson.createdAt);
  const role = page.params.role;

  const href =
    role === "t" ? `/t/lessons/l/${lesson.id}` : `/s/lessons/l/${lesson.id}`;
</script>

<CardClickable {href}>
  <H2>
    {#if role === "s"}
      {formattedDate}
    {:else}
      {lesson.title}
    {/if}
  </H2>

  <div class="flex flex-col">
    <h2
      class="text-lg font-semibold
			"
    >
      {lesson.topic}
    </h2>

    <p class=" text-sm/relaxed text-stone-600 lg:text-base/relaxed">
      {@html rendered}
    </p>
  </div>
</CardClickable>
