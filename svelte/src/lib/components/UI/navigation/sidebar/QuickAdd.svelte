<script lang="ts">
  import UniButton from "$lib/components/UI/forms/buttons/UniButton.svelte";
  import { BookOpenCheck, ListTodo, Plus } from "@lucide/svelte";
  import { enhance } from "$app/forms";
  import HStack from "../../layout/HStack.svelte";
  import { page } from "$app/state";
  import { Merger } from "../../toolbar";

  const role = page.params.role;
</script>

<form action="/{role}/tasks?/new" method="POST" class="" use:enhance>
  <Merger>
    <UniButton
      Icon={Plus}
      type="submit"
      variant="primary"
      iconOnly={false}
      content="Задание"
    ></UniButton>
  </Merger>
</form>
<form action="/{role}/lessons?/new" method="POST" class="" use:enhance>
  <Merger>
    <UniButton
      Icon={Plus}
      type="submit"
      variant="primary"
      content="Занятие"
      iconOnly={false}
    ></UniButton>
  </Merger>
</form>
