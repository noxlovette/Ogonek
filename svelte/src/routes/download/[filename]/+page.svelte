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
    class="dark:bg-milk-900 max-w-md space-y-6 rounded-xl bg-white p-6 shadow-md md:w-full md:p-8"
  >
    <div class="flex items-center justify-center space-x-2">
      <BookOpen class="dark:text-milk-700 text-cacao-500 h-8 w-8" />
      <h1 class="text-milk-800 text-2xl font-bold dark:text-inherit">
        Homework Time!
      </h1>
    </div>

    <div class="space-y-4">
      {#await data.body}
        <div class="flex items-center justify-center space-x-3">
          <Loader2
            class="text-cacao-500 dark:text-milk-200 h-8 w-8 animate-spin"
          />
          <p class="text-milk-600">Loading...</p>
        </div>
      {:then body}
        <div class="flex items-center justify-center space-x-3">
          <FileCheck class="text-cacao-500 dark:text-milk-200 h-8 w-8" />
          <p class="text-milk-600 dark:text-milk-100">Enjoy!</p>
        </div>
      {:catch error}
        <p>Something went wrong: {error.message}</p>
      {/await}

      <button
        onclick={download}
        class="bg-cacao-500 hover:bg-cacao-600 dark:bg-milk-800 dark:hover:bg-milk-700 flex w-full items-center justify-center space-x-2 rounded-lg px-4 py-2 font-medium text-white transition-colors duration-200"
      >
        <Download class="h-4 w-4" />
        <span>Download Again</span>
      </button>

      <button
        onclick={goBack}
        class="bg-milk-100 hover:bg-milk-200 text-milk-700 mt-2 flex w-full items-center justify-center space-x-2 rounded-lg px-4 py-2 font-medium transition-colors duration-200"
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
