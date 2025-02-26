<script lang="ts">
  import { ListTodo } from "lucide-svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import { user } from "$lib/stores";

  let href = $user.role === "teacher" ? "/t/tasks" : "/s/tasks";
</script>

<SidebarItem {href} Icon={ListTodo} name="Tasks" />
