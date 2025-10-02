<script lang="ts">
  import { Download } from "@lucide/svelte";
  import UniButton from "./UniButton.svelte";
  import type { URLResponse } from "$lib/types";
  import Caption1 from "$lib/components/typography/Caption1.svelte";

  let downloadQueue = $state<URLResponse[]>([]);
  let currentDownloads = $state(0);
  let totalDownloads = $state(0);
  let isDownloading = $state(false);
  let hasTriggered = $state(false);
  const MAX_CONCURRENT = 3;

  const {
    content = "Загрузить задание",
    urls,
  }: {
    content?: string;
    urls?: URLResponse[];
  } = $props();

  $effect(() => {
    if (urls && urls.length > 0 && !hasTriggered) {
      hasTriggered = true;
      startBulkDownload(urls);
    }
  });

  async function downloadFile(url: string) {
    const iframe = document.createElement("iframe");
    iframe.style.display = "none";
    iframe.src = url;
    document.body.appendChild(iframe);

    await new Promise((resolve) => setTimeout(resolve, 1000));
    document.body.removeChild(iframe);
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

  async function startBulkDownload(urls: URLResponse[]) {
    downloadQueue = [...urls];
    totalDownloads = 0;
    isDownloading = true;

    for (let i = 0; i < MAX_CONCURRENT; i++) {
      processQueue();
    }
  }
</script>

<UniButton type="submit" Icon={Download} formaction="?/downloadAll" {content} />

{#if urls}
  <Caption1 styling="self-center mr-3">
    {totalDownloads} / {downloadQueue.length +
      currentDownloads +
      totalDownloads}
  </Caption1>
{/if}
