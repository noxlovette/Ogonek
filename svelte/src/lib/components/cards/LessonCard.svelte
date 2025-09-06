<script lang="ts">
  import type { LessonSmall } from "$lib/types";
  import { formatDate } from "$lib/utils";
  import CardClickable from "./CardClickable.svelte";
  import { page } from "$app/state";
  import Badge from "./Badge.svelte";
  import { Title3 } from "../typography";
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
  <Title3>
    {title}
  </Title3>
  <Badge>{badgeText}</Badge>
  <SeenBadge seen={lesson.seen || true} />
</CardClickable>
