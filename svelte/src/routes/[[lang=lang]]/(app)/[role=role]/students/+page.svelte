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
  <LargeTitle>Students</LargeTitle>
  <Divider />
  <Merger>
    <StudentAdder />
  </Merger>
</Toolbar>
<Table config={studentConfig} {href} items={students} {students} />

<svelte:head>
  <title>Students</title>
</svelte:head>
