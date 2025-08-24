<script lang="ts">
  import { H3 } from "$lib/components/typography";
  import Group from "./Group.svelte";
  import UniButton from "$lib/components/UI/UniButton.svelte";
  import { BookOpenCheck, ListTodo, WholeWord } from "lucide-svelte";
  import { enhance } from "$app/forms";
</script>

<Group>
  <H3>Quick Add</H3>

  <div class="mt-4 grid w-full grid-cols-1 gap-4">
    <form action="/t/tasks?/new" method="POST" class="" use:enhance>
      <UniButton
        size="sm"
        Icon={ListTodo}
        fullWidth={true}
        type="submit"
        variant="primary">Add Task</UniButton
      >
    </form>
    <form action="/t/lessons?/new" method="POST" class="" use:enhance>
      <UniButton
        size="sm"
        Icon={BookOpenCheck}
        fullWidth={true}
        type="submit"
        variant="primary">Add Lesson</UniButton
      >
    </form>
    <form action="/t/flashcards?/new" method="POST" class="" use:enhance>
      <UniButton
        size="sm"
        Icon={WholeWord}
        fullWidth={true}
        type="submit"
        variant="primary">Add Deck</UniButton
      >
    </form>
  </div>
</Group>
