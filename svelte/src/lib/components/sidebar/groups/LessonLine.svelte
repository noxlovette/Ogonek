<script lang="ts">
  import { formatDate } from "@noxlovette/svarog";
  import type { Lesson } from "$lib/types";

  import { page } from "$app/state";
  import SidebarItem from "../elements/SidebarItem.svelte";
  import { Library } from "lucide-svelte";
  interface Props {
    lesson: Lesson;
  }

  let { lesson }: Props = $props();
  const href = `/${page.params.role}/lessons/l/${lesson.id}`;
  const formattedDate = formatDate(lesson.createdAt);
</script>

<SidebarItem Icon={Library} {href} name={formattedDate}></SidebarItem>
