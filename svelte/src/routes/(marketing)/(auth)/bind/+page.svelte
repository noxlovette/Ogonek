<script lang="ts">
  import { enhance } from "$app/forms";
  import { user } from "$lib/stores";
  import { Body, Merger, Title1, UniButton } from "$lib/components";
  import { enhanceForm } from "$lib/utils";
  import { Cable } from "lucide-svelte";

  let { data } = $props();

  const { inviter } = data;
</script>

<form
  method="POST"
  class="flex flex-col gap-2 md:gap-3 lg:gap-4"
  use:enhance={enhanceForm({
    messages: {
      redirect: "Bound to teacher",
    },
    navigate: true,
  })}
>
  <input type="hidden" name="userId" value={$user.id} />

  <Title1>{inviter?.name}</Title1>
  <Body>wants to add you to their students</Body>

  <Merger>
    <UniButton iconOnly={false} variant="primary" type="submit" Icon={Cable}
      >Connect</UniButton
    >
  </Merger>
</form>

<svelte:head>
  <title>Connect to Teacher</title>
</svelte:head>
