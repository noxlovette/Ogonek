<script lang="ts">
  import { ListTodo } from "lucide-svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { user } from "$lib/stores";
  import { getContext } from "svelte";

  let href = $user.role === "teacher" ? "/t/tasks" : "/s/tasks";
  const taskCount = getContext<number>("taskCount");
</script>

<SidebarItem {href} Icon={ListTodo} name="Tasks" badge={taskCount} />
