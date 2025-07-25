<script lang="ts">
  import type { LessonSmall } from "$lib/types";
  import { formatDate } from "@noxlovette/svarog";
  import CardClickable from "./CardClickable.svelte";
  import { page } from "$app/state";
  import Badge from "./Badge.svelte";
  import { H3 } from "../typography";
  import SeenBadge from "./SeenBadge.svelte";
  interface Props {
    lesson: LessonSmall;
  }
  let { lesson }: Props = $props();

  const formattedDate = formatDate(lesson.createdAt);
  const role = page.params.role;

  const href =
    role === "t" ? `/t/lessons/${lesson.id}` : `/s/lessons/${lesson.id}`;

  const badgeText = role == "t" ? lesson.title : formattedDate;
  const title = lesson.topic;
</script>

<CardClickable {href}>
  <H3>
    {title}
  </H3>
  <Badge {badgeText} />
  <SeenBadge seen={lesson.seen} />
</CardClickable>
