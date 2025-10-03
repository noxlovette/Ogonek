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
    Title1,
    TableBody,
    TableRow,
    Headline,
  } from "$lib/components";
  import type { Student } from "$lib/types";
  import { m } from "$lib/paraglide/messages";

  let { data }: { data: PageData } = $props();

  const { students } = data;
</script>

<Toolbar>
  <LargeTitle>{m.mellow_stout_fireant_assure()}</LargeTitle>
  <Divider />
  <Merger>
    <StudentAdder />
  </Merger>
</Toolbar>

{#if data.students.length < 1}
  <EmptySpace>
    <Title1>{m.empty()}</Title1>
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
  <title>{m.mellow_stout_fireant_assure()}</title>
</svelte:head>
