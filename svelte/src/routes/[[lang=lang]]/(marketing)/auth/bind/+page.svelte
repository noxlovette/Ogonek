<script lang="ts">
  import { enhance } from "$app/forms";
  import { user } from "$lib/stores";
  import { Caption, H1, H3, Toolbar, UniButton } from "$lib/components";
  import { enhanceForm } from "$lib/utils";
  import { Cable } from "lucide-svelte";

  let { data } = $props();

  const { inviter } = data;
</script>

<div class="">
  <form
    method="POST"
    use:enhance={enhanceForm({
      messages: {
        redirect: "Bound to teacher",
      },
      navigate: true,
    })}
  >
    <input type="hidden" name="userId" value={$user.id} />
    <div
      class="flex max-w-md flex-col items-center justify-center space-y-3 rounded-sm px-6 py-4 text-center shadow-sm"
    >
      <Toolbar>
        <H1 styling="text-center">{inviter.name}</H1>
      </Toolbar>

      <H3>wants to add you to their students</H3>

      <Caption>If you don't know them do not connect</Caption>
      <UniButton styling="mt-4" variant="primary" type="submit" Icon={Cable}
        >Start Work</UniButton
      >
    </div>
  </form>
</div>
<svelte:head>
  <title>Connect to Teacher</title>
</svelte:head>
