<script lang="ts">
  import { BookOpenCheck } from "lucide-svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { user } from "$lib/stores";
  import { getContext } from "svelte";
  import { m } from "$lib/paraglide/messages";

  let href = $user.role === "teacher" ? "/t/lessons" : "/s/lessons";

  const lessonCount = getContext<number>("lessonCount");
</script>

<SidebarItem
  {href}
  Icon={BookOpenCheck}
  name={m.lessons()}
  badge={lessonCount}
/>
