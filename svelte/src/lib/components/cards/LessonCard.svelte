<script lang="ts">
  import type { Lesson } from "$lib/types";
  import { parseMarkdown } from "$lib/utils";
  import { formatDate } from "@noxlovette/svarog";
  import { user } from "$lib/stores";
  import CardClickable from "./CardClickable.svelte";
  import { H2 } from "../typography";
  import { onMount } from "svelte";
  interface Props {
    lesson: Lesson;
  }

  onMount(async () => {
    rendered = await parseMarkdown(lesson.markdown);
  });

  let { lesson }: Props = $props();
  let rendered = $state(lesson.markdown);

  console.log(lesson.createdAt);

  const formattedDate = formatDate(lesson.createdAt);
  let href =
    $user.role === "teacher"
      ? `/t/lessons/l/${lesson.id}`
      : `/s/lessons/l/${lesson.id}`;
</script>

<CardClickable {href}>
  <H2>
    {formattedDate}
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
