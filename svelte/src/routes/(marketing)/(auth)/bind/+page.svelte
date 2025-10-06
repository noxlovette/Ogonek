<script lang="ts">
  import { enhance } from "$app/forms";
  import { user } from "$lib/stores";
  import { Body, Merger, Title1, UniButton } from "$lib/components";
  import { enhanceForm } from "$lib/utils";
  import { Cable } from "@lucide/svelte";

  let { data } = $props();

  const { inviter } = data;
</script>

<form
  method="POST"
  class="gap-default flex flex-col items-center justify-center"
  use:enhance={enhanceForm({
    messages: {
      redirect: "Вы связаны узами Огонька",
    },
    navigate: true,
  })}
>
  <input type="hidden" name="userId" value={$user.id} />

  <Title1>{inviter?.name}</Title1>
  <Body>хочет с вами работать</Body>

  <Merger>
    <UniButton
      content="Присоединиться"
      iconOnly={false}
      variant="primary"
      type="submit"
      Icon={Cable}
    ></UniButton>
  </Merger>
</form>

<svelte:head>
  <title>Привязка к преподавателю</title>
</svelte:head>
