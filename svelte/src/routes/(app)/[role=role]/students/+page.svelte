<script lang="ts">
  import type { PageData } from "./$types";
  import { Table, H1, HeaderEmbellish, StudentAdder } from "$lib/components";
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

  let href = "/t/students/st";
</script>

<HeaderEmbellish>
  <div class="flex flex-col gap-3 md:flex-row md:gap-4">
    <H1>Students</H1>
    <StudentAdder></StudentAdder>
  </div>
</HeaderEmbellish>
<Table config={studentConfig} {href} items={students} {students} />

<svelte:head>
  <title>Students</title>
</svelte:head>
