<script lang="ts">
  import { ListTodo } from "lucide-svelte";
  import Base from "./Base.svelte";
  import { user } from "$lib/stores";

  let href = $user.role === "teacher" ? "/t/tasks" : "/s/tasks";
</script>

<Base {href}>
  <ListTodo class="size-6 md:mr-2 lg:size-7 xl:size-8" />
  <p class="hidden md:flex">Tasks</p>
</Base>
