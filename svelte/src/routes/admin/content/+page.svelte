<script lang="ts">
  import { Toolbar, Table, Divider, Merger } from "$lib/components";
  import type { PageProps } from "./$types";
  import { enhanceForm } from "$lib/utils";
  import LargeTitle from "$lib/components/typography/LargeTitle.svelte";
  import { enhance } from "$app/forms";
  import NewButton from "$lib/components/UI/forms/buttons/NewButton.svelte";

  let { data }: PageProps = $props();
</script>

<Toolbar>
  <LargeTitle>Content</LargeTitle>
  <Divider></Divider>

  <Merger>
    <form
      action="?/new"
      method="post"
      use:enhance={enhanceForm({
        messages: {
          redirect: "New content added",
        },
        navigate: true,
      })}
    >
      <NewButton />
    </form>
  </Merger>
</Toolbar>
<Table>Coming soon</Table>
