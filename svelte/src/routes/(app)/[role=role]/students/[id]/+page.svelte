<script lang="ts">
  import {
    EmptySpace,
    LargeTitle,
    Title3,
    Toolbar,
    Divider,
  } from "$lib/components";
  import Title2 from "$lib/components/typography/Title2.svelte";
  import EditButton from "$lib/components/UI/forms/buttons/EditButton.svelte";
  let { data } = $props();

  const { student, rendered } = data;
</script>

<svelte:head>
  <title>{student.name}</title>
</svelte:head>
<Toolbar>
  <LargeTitle>{student.name}</LargeTitle>
  <Title3>
    {student.email}
  </Title3>
  <Divider />
  <EditButton href="/t/students/{student.id}/edit" />
</Toolbar>
<Title2>Notes</Title2>
{#if student.markdown}
  <div class="">
    {@html rendered}
  </div>
{:else}
  <EmptySpace>
    <Title3>No Notes</Title3>
  </EmptySpace>
{/if}
