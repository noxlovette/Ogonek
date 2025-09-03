<script lang="ts">
  import UniButton from "$lib/components/UI/forms/buttons/UniButton.svelte";
  import { BookOpenCheck, ListTodo, WholeWord } from "lucide-svelte";
  import { enhance } from "$app/forms";
  import HStack from "../../HStack.svelte";
</script>

<HStack styling="gap-2">
  <form action="/t/tasks?/new" method="POST" class="" use:enhance>
    <UniButton Icon={ListTodo} type="submit" variant="primary" iconOnly={false}
      >Add Task</UniButton
    >
  </form>
  <form action="/t/lessons?/new" method="POST" class="" use:enhance>
    <UniButton
      Icon={BookOpenCheck}
      type="submit"
      variant="primary"
      iconOnly={false}>Add Lesson</UniButton
    >
  </form>
  <form action="/t/flashcards?/new" method="POST" class="" use:enhance>
    <UniButton Icon={WholeWord} type="submit" variant="primary" iconOnly={false}
      >Add Deck</UniButton
    >
  </form>
</HStack>
