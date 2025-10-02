<script lang="ts">
  import UniButton from "$lib/components/UI/forms/buttons/UniButton.svelte";
  import { BookOpenCheck, GraduationCap, ListTodo } from "@lucide/svelte";
  import { enhance } from "$app/forms";
  import { m } from "$lib/paraglide/messages";
  import HStack from "../../layout/HStack.svelte";
  import { page } from "$app/state";

  const role = page.params.role;
</script>

<HStack styling="gap-2">
  <form action="/{role}/tasks?/new" method="POST" class="" use:enhance>
    <UniButton
      Icon={ListTodo}
      type="submit"
      variant="primary"
      iconOnly={false}
      content={m.brave_vexed_capybara_aid()}
    ></UniButton>
  </form>
  <form action="/{role}/lessons?/new" method="POST" class="" use:enhance>
    <UniButton
      Icon={BookOpenCheck}
      type="submit"
      variant="primary"
      content={m.game_green_ape_fond()}
      iconOnly={false}
    ></UniButton>
  </form>
</HStack>
