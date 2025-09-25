<script lang="ts">
  import UniButton from "$lib/components/UI/forms/buttons/UniButton.svelte";
  import { BookOpenCheck, ListTodo, WholeWord } from "lucide-svelte";
  import { enhance } from "$app/forms";
  import { m } from "$lib/paraglide/messages";
  import HStack from "../../layout/HStack.svelte";
</script>

<HStack styling="gap-2">
  <form action="/t/tasks?/new" method="POST" class="" use:enhance>
    <UniButton
      Icon={ListTodo}
      type="submit"
      variant="primary"
      iconOnly={false}
      content={m.brave_vexed_capybara_aid()}
    ></UniButton>
  </form>
  <form action="/t/lessons?/new" method="POST" class="" use:enhance>
    <UniButton
      Icon={BookOpenCheck}
      type="submit"
      variant="primary"
      content={m.game_green_ape_fond()}
      iconOnly={false}
    ></UniButton>
  </form>
  <form action="/t/flashcards?/new" method="POST" class="" use:enhance>
    <UniButton
      Icon={WholeWord}
      type="submit"
      variant="primary"
      iconOnly={false}
      content={m.sour_tough_goat_cure()}
    ></UniButton>
  </form>
</HStack>
