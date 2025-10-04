<script lang="ts">
  import { enhance } from "$app/forms";
  import { user } from "$lib/stores";
  import { Body, Merger, Title1, UniButton } from "$lib/components";
  import { enhanceForm } from "$lib/utils";
  import { Cable } from "@lucide/svelte";
  import { m } from "$lib/paraglide/messages.js";

  let { data } = $props();

  const { inviter } = data;
</script>

<form
  method="POST"
  class="gap-default flex flex-col items-center justify-center"
  use:enhance={enhanceForm({
    messages: {
      redirect: m.trite_bad_beaver_comfort(),
    },
    navigate: true,
  })}
>
  <input type="hidden" name="userId" value={$user.id} />

  <Title1>{inviter?.name}</Title1>
  <Body>{m.noble_aloof_bulldog_swim()}</Body>

  <Merger>
    <UniButton
      content={m.fun_level_beetle_grow()}
      iconOnly={false}
      variant="primary"
      type="submit"
      Icon={Cable}
    ></UniButton>
  </Merger>
</form>

<svelte:head>
  <title>Connect to Teacher</title>
</svelte:head>
