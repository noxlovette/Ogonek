<script lang="ts">
  import { onMount } from "svelte";
  import {
    Download,
    ArrowLeft,
    BookOpen,
    Loader2,
    FileCheck,
  } from "lucide-svelte";
  import { stripUUID } from "$lib/utils";
  import { afterNavigate, goto } from "$app/navigation";
  import { navigating } from "$app/state";

  let { data } = $props();
  const { filename } = data;
  let body: ArrayBuffer | undefined = $state();

  onMount(async () => {
    body = await data.body;
  });

  $effect(() => {
    if (body?.byteLength) download();
  });
  function download() {
    if (body) {
      const blob = new Blob([body], {
        type: "application/octet-stream",
      });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = stripUUID(filename);
      a.click();
      URL.revokeObjectURL(url);
    }
  }
  function goBack() {
    window.history.back();
  }
</script>

<div class="flex size-full h-max justify-center md:items-center">
  <div
    class="max-w-md space-y-6 rounded-xl bg-white p-6 shadow-sm md:w-full md:p-8 dark:bg-stone-900"
  >
    <div class="flex items-center justify-center space-x-2">
      <BookOpen class="text-cacao-500 h-8 w-8 dark:text-stone-700" />
      <h1 class="text-2xl font-bold text-stone-800 dark:text-inherit">
        Homework Time!
      </h1>
    </div>

    <div class="space-y-4">
      {#await data.body}
        <div class="flex items-center justify-center space-x-3">
          <Loader2
            class="text-cacao-500 h-8 w-8 animate-spin dark:text-stone-200"
          />
          <p class="text-stone-600">Loading...</p>
        </div>
      {:then body}
        <div class="flex items-center justify-center space-x-3">
          <FileCheck class="text-cacao-500 h-8 w-8 dark:text-stone-200" />
          <p class="text-stone-600 dark:text-stone-100">Enjoy!</p>
        </div>
      {:catch error}
        <p>Something went wrong: {error.message}</p>
      {/await}

      <button
        onclick={download}
        class="bg-cacao-500 hover:bg-cacao-600 flex w-full items-center justify-center space-x-2 rounded-lg px-4 py-2 font-medium text-white transition-colors duration-200 dark:bg-stone-800 dark:hover:bg-stone-700"
      >
        <Download class="h-4 w-4" />
        <span>Download Again</span>
      </button>

      <button
        onclick={goBack}
        class="mt-2 flex w-full items-center justify-center space-x-2 rounded-lg bg-stone-100 px-4 py-2 font-medium text-stone-700 transition-colors duration-200 hover:bg-stone-200"
      >
        <ArrowLeft class="h-4 w-4" />
        <span>Go Back</span>
      </button>
    </div>
  </div>
</div>

<svelte:head>
  <title>Download | Ogonek</title>
</svelte:head>
