<script lang="ts">
  import {
    EmptySpace,
    LargeTitle,
    Title3,
    Toolbar,
    Divider,
    Title2,
    EditButton,
    Merger,
    HStack,
    Callout,
  } from "$lib/components";
  import texts from "$lib/texts.js";
  let { data } = $props();

  const { student, rendered } = data;
</script>

<svelte:head>
  <title>{student.name}</title>
</svelte:head>
<Toolbar>
  <HStack>
    <LargeTitle>{student.name}</LargeTitle>
    <Callout>
      {student.email}
    </Callout>
  </HStack>
  <Divider />
  <Merger>
    <EditButton href="/t/students/{student.id}/edit" />
  </Merger>
</Toolbar>
<Title2>Заметки</Title2>
{#if student.markdown}
  <div class="">
    {@html rendered}
  </div>
{:else}
  <EmptySpace>
    {texts.table.empty}
  </EmptySpace>
{/if}
