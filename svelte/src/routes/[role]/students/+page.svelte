<script lang="ts">
  import type { PageData } from "./$types";
  import {
    Table,
    H1,
    UniButton,
    HeaderEmbellish,
    StudentAdder,
  } from "$lib/components";
  import type { Student, TableConfig } from "$lib/types";
  import { enhance } from "$app/forms";
  import { notification } from "$lib/stores";
  import { PersonStanding } from "lucide-svelte";
  import { enhanceForm } from "$lib/utils";

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
  <H1>Students</H1>
  <StudentAdder></StudentAdder>
</HeaderEmbellish>
<Table config={studentConfig} {href} items={students} {students} />

<svelte:head>
  <title>Students</title>
</svelte:head>
