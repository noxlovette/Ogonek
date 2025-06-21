<script lang="ts">
  import type { LessonSmall } from "$lib/types";

  import { page } from "$app/state";
  import Badge from "$lib/components/cards/Badge.svelte";
  import { formatDate } from "@noxlovette/svarog";
  interface Props {
    lesson: LessonSmall;
  }

  let { lesson }: Props = $props();
  const href = `/${page.params.role}/lessons/l/${lesson.id}`;
</script>

<a
  class="flex-inline items-end justify-between space-y-1.5 transition-all hover:scale-[1.02]"
  {href}
>
  <p>
    {lesson.topic}
  </p>
  <Badge badgeText={formatDate(lesson.createdAt)}></Badge>
</a>
