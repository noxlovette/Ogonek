<script lang="ts">
  import { Download, FileDown } from "lucide-svelte";
  import UniButton from "./UniButton.svelte";
  import type { BatchPresign } from "$lib/types";

  let downloadQueue = $state<BatchPresign[]>([]);
  let currentDownloads = $state(0);
  let totalDownloads = $state(0);
  let isDownloading = $state(false);
  let hasTriggered = $state(false);
  let pdfDownloaded = $state(false);
  const MAX_CONCURRENT = 3;

  const {
    content = "Загрузить задание",
    urls,
    pdfBlob,
  }: {
    content?: string;
    urls?: BatchPresign[];
    pdfBlob?: Blob;
  } = $props();

  // Trigger download des presigned URLs
  $effect(() => {
    if (urls && urls.length > 0 && !hasTriggered) {
      hasTriggered = true;
      startBulkDownload(urls);
    }
  });

  // Trigger download du PDF séparément
  $effect(() => {
    if (pdfBlob && !pdfDownloaded) {
      console.log("effect triggered");
      pdfDownloaded = true;
      downloadBlob(pdfBlob, `task-${Date.now()}.pdf`);
    }
  });

  function downloadBlob(blob: Blob, filename: string) {
    console.log("blob triggered");
    const link = document.createElement("a");
    link.href = URL.createObjectURL(blob);
    link.download = filename;
    link.click();
    URL.revokeObjectURL(link.href);
  }

  async function downloadFile(url: string, filename?: string) {
    try {
      const response = await fetch(url);
      const blob = await response.blob();
      const link = document.createElement("a");
      link.href = URL.createObjectURL(blob);
      link.download = filename || `file-${Date.now()}`;
      link.click();
      URL.revokeObjectURL(link.href);
    } catch (err) {
      console.error("Download failed:", err);
    }
  }

  async function processQueue() {
    if (downloadQueue.length === 0 || currentDownloads >= MAX_CONCURRENT) {
      if (currentDownloads === 0) isDownloading = false;
      return;
    }

    const url = downloadQueue.shift()!;
    currentDownloads++;
    await downloadFile(url.url);
    currentDownloads--;
    totalDownloads++;

    processQueue();
  }

  async function startBulkDownload(urls: BatchPresign[]) {
    downloadQueue = [...urls];
    totalDownloads = 0;
    isDownloading = true;

    for (let i = 0; i < MAX_CONCURRENT; i++) {
      processQueue();
    }
  }
</script>

<UniButton type="submit" Icon={Download} formaction="?/downloadAll" {content} />

<UniButton
  type="submit"
  Icon={FileDown}
  formaction="?/downloadPdf"
  content="Télécharger PDF"
/>

{#if isDownloading}
  <div class="text-sm text-stone-600">
    Téléchargement... {totalDownloads} / {downloadQueue.length +
      currentDownloads +
      totalDownloads}
  </div>
{/if}
