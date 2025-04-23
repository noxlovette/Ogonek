<script lang="ts">
  import type { LessonSmall } from "$lib/types";
  import { parseMarkdown } from "$lib/utils";
  import { formatDate } from "@noxlovette/svarog";
  import CardClickable from "./CardClickable.svelte";
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

  const title = role == "t" ? lesson.title : formattedDate;
  const subtitle = lesson.topic;
  const caption = $derived(rendered);
</script>

<CardClickable {href} {title} {subtitle} {caption}></CardClickable>
