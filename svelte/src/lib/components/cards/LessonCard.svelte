<script lang="ts">
  import type { LessonSmall } from "$lib/types";
  import { formatDate } from "@noxlovette/svarog";
  import CardClickable from "./CardClickable.svelte";
  import { page } from "$app/state";
  interface Props {
    lesson: LessonSmall;
  }
  let { lesson }: Props = $props();

  const formattedDate = formatDate(lesson.createdAt);
  const role = page.params.role;

  const href =
    role === "t" ? `/t/lessons/l/${lesson.id}` : `/s/lessons/l/${lesson.id}`;

  const badgeText = role == "t" ? lesson.title : formattedDate;
  const title = lesson.topic;
</script>

<CardClickable {href} {title} {badgeText}></CardClickable>
