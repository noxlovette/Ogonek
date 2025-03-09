<script lang="ts">
  import { H1, UniButton } from "$lib/components";
  import type { Student } from "$lib/types";
  import type { PageData } from "./$types";
  import { Pencil } from "lucide-svelte";
  let { data }: { data: PageData } = $props();

  const { student, rendered }: { student: Student; rendered: String } = data;
</script>

<svelte:head>
  <title>{student.name}</title>
</svelte:head>
<div class="flex items-baseline space-x-4">
  <H1>{student.name}</H1>

  <UniButton
    Icon={Pencil}
    href="/t/students/st/{student.id}/edit"
    variant="outline">Edit</UniButton
  >
</div>
<div class="flex space-x-4">
  <div class="space-y-2">
    <p class="block font-medium text-stone-700">Email</p>
    <h3 class="min-w-48">
      {student.email}
    </h3>
  </div>
</div>
<h3 class="text-2xl font-bold">Notes</h3>
{#if student.markdown}
  <div class="markdown rounded-lg p-4 ring ring-stone-200">
    {@html rendered}
  </div>
{:else}
  <p>No notes yet</p>
{/if}
