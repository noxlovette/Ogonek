<script lang="ts">
  import {
    EmptySpace,
    LargeTitle,
    Title3,
    Toolbar,
    Divider,
    Title2,
    EditButton,
  } from "$lib/components";
  import texts from "$lib/texts.js";
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
