<script lang="ts">
  import { ButtonEdit, H1 } from "$lib/components";
  import type { Student } from "$lib/types";
  import type { PageData } from "./$types";
  let { data }: { data: PageData } = $props();

  const { student, rendered }: { student: Student; rendered: String } = data;
</script>

<svelte:head>
  <title>{student.name}</title>
</svelte:head>
<div class="flex items-baseline space-x-4">
  <H1>{student.name}</H1>

  <ButtonEdit href="/t/students/st/{student.id}/edit" />
</div>
<div class="flex space-x-4">
  <div class="space-y-2">
    <p class="text-milk-700 block font-medium">Email</p>
    <h3 class="min-w-48">
      {student.email}
    </h3>
  </div>
</div>
<h3 class="text-2xl font-bold">Notes</h3>
{#if student.markdown}
  <div class="markdown ring-milk-200 rounded-lg p-4 ring-2">
    {@html rendered}
  </div>
{:else}
  <p>No notes yet</p>
{/if}
