<script lang="ts">
  import { formatDate } from "@noxlovette/svarog";
  import CardClickable from "./CardClickable.svelte";
  import { user } from "$lib/stores";
  import { onMount } from "svelte";
  import { parseMarkdown } from "@noxlovette/svarog";

  onMount(async () => {
    rendered = await parseMarkdown(task.markdown.slice(0, 100));
  });
  let { task } = $props();

  let overdue = $derived(new Date(task.dueDate) < new Date());

  let rendered = $state(task.markdown);
  const formattedDate = formatDate(task.dueDate);
  let href =
    $user.role === "teacher"
      ? `/t/tasks/t/${task.id}`
      : `/s/tasks/t/${task.id}`;

  const title = task.title;
  const subtitle = `Due ${formattedDate}`;
  const caption = $derived(rendered);
</script>

<CardClickable
  {href}
  {title}
  {subtitle}
  {caption}
  styling={overdue ? "ring-offset-2 ring-offset-red-400/50" : ""}
/>
