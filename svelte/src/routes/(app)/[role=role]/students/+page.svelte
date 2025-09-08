<script lang="ts">
  import type { PageData } from "./$types";
  import {
    Table,
    LargeTitle,
    Toolbar,
    StudentAdder,
    Divider,
    Merger,
  } from "$lib/components";
  import type { Student, TableConfig } from "$lib/types";
  import { m } from "$lib/paraglide/messages";

  let { data }: { data: PageData } = $props();

  const { students } = data;

  const studentConfig: TableConfig<Student> = {
    columns: [
      { key: "name", label: "Name" },
      { key: "username", label: "Username" },
      { key: "email", label: "Email" },
      { key: "markdown", label: "Markdown" },
    ],
  };

  let href = "/t/students";
</script>

<Toolbar>
  <LargeTitle>{m.mellow_stout_fireant_assure()}</LargeTitle>
  <Divider />
  <Merger>
    <StudentAdder />
  </Merger>
</Toolbar>
<Table config={studentConfig} {href} items={students} {students} />

<svelte:head>
  <title>{m.mellow_stout_fireant_assure()}</title>
</svelte:head>
