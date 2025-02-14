<script lang="ts">
  import { onMount } from "svelte";
  import { Download, ArrowLeft, BookOpen, Loader2 } from "lucide-svelte";
  import { stripUUID } from "$lib/utils";

  let { data } = $props();
  const { body, headers, filename } = data;

  let isDownloading = $state(true);
  let downloadStarted = $state(false);

  onMount(() => {
    try {
      const blob = new Blob([body], { type: headers["Content-Type"] });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = stripUUID(filename);
      a.click();
      URL.revokeObjectURL(url);
      downloadStarted = true;
    } catch (e) {
      isDownloading = false;
    }
  });

  function downloadAgain() {
    isDownloading = true;
    const blob = new Blob([body], {
      type: headers["Content-Type"] || "application/octet-stream",
    });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = stripUUID(filename);
    a.click();
    URL.revokeObjectURL(url);
  }

  function goBack() {
    window.history.back();
  }
</script>

<div class=" flex size-full items-center justify-center">
  <div
    class="dark:bg-milk-900 w-full max-w-md space-y-6 rounded-xl bg-white p-8 shadow-md"
  >
    <div class="flex items-center justify-center space-x-2">
      <BookOpen class="dark:text-milk-700 text-cacao-500 h-8 w-8" />
      <h1 class="text-milk-800 text-2xl font-bold dark:text-inherit">
        Homework Time! 📚
      </h1>
    </div>

    <div class="space-y-4">
      {#if isDownloading}
        <div class="flex flex-col items-center space-y-3">
          <Loader2
            class="text-cacao-500 dark:text-cacao-700 h-8 w-8 animate-spin"
          />
          <p class="text-milk-600">Getting your homework ready...</p>
        </div>
      {:else}
        <div class="flex flex-col items-center space-y-3">
          <Download class="text-cacao-500 dark:text-cacao-800 h-8 w-8" />
          <p class="text-milk-600">
            {downloadStarted ? "Almost there! 🚀" : "Ready to download! 🎉"}
          </p>
        </div>
      {/if}

      <button
        onclick={downloadAgain}
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

    <p class="text-milk-500 text-center text-sm">
      Pro tip: Your teacher won't believe how fast you did this! 😉
    </p>
  </div>
</div>

<svelte:head>
  <title>Download | Firelight</title>
</svelte:head>
