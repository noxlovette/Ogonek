<script lang="ts">
  import type { PageData } from "./$types";
  import { page as sveltePage } from "$app/state";
  import {
    Table,
    LargeTitle,
    Toolbar,
    StudentAdder,
    Divider,
    Merger,
    EmptySpace,
    TableBody,
    TableRow,
    Headline,
  } from "$lib/components";
  import texts from "$lib/texts";

  let { data }: { data: PageData } = $props();

  const { students } = data;
</script>

<Toolbar>
  <LargeTitle>Ученики</LargeTitle>
  <Divider />
  <Merger>
    <StudentAdder />
  </Merger>
</Toolbar>

{#if data.students.length < 1}
  <EmptySpace>
    {texts.table.empty}
  </EmptySpace>
{/if}
<Table>
  <TableBody>
    {#each students as student (student.id)}
      <TableRow href={`/${sveltePage.params.role}/students/${student.id}`}>
        <Headline>
          {student.name}
        </Headline>
      </TableRow>
    {/each}
  </TableBody>
</Table>
<svelte:head>
  <title>Ученики</title>
</svelte:head>
