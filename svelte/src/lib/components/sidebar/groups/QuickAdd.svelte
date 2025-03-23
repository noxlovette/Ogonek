<script lang="ts">
  import H2 from "$lib/components/typography/H2.svelte";
  import Group from "./Group.svelte";
  import UniButton from "$lib/components/UI/UniButton.svelte";
  import { BookOpenCheck, ListTodo, WholeWord } from "lucide-svelte";
  import { enhance } from "$app/forms";
</script>

<Group>
  <H2>Quick Add</H2>

  <div class="mt-4 grid w-full grid-cols-1 gap-4">
    <form action="/t/tasks?/new" method="POST" class="" use:enhance>
      <UniButton
        size="sm"
        Icon={ListTodo}
        fullWidth={true}
        type="submit"
        variant="outline">Add Task</UniButton
      >
    </form>
    <form action="/t/lessons?/new" method="POST" class="" use:enhance>
      <UniButton
        size="sm"
        Icon={BookOpenCheck}
        fullWidth={true}
        type="submit"
        variant="outline">Add Lesson</UniButton
      >
    </form>
    <form action="/t/words?/new" method="POST" class="" use:enhance>
      <UniButton
        size="sm"
        Icon={WholeWord}
        fullWidth={true}
        type="submit"
        variant="outline">Add Deck</UniButton
      >
    </form>
  </div>
</Group>
