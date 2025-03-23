<script lang="ts">
  import { notification } from "$lib/stores/notification";
  import { enhanceForm } from "$lib/utils";
  import { isLoading } from "$lib/stores";
  import { enhance } from "$app/forms";
  import type { FileSmall } from "$lib/types";
  import { formatFileSize, getFileExtension } from "$lib/utils";
  import {
    FileText,
    Image,
    FileAudio,
    FileVideo,
    File,
    Loader2,
  } from "lucide-svelte";
  import H3 from "../typography/H3.svelte";

  let { file }: { file: FileSmall } = $props();
  let downloading = $state(false);
</script>

{#snippet icon(mimeType: string)}
  {#if mimeType.startsWith("image/")}
    <Image class="size-10 text-stone-600" />
  {:else if mimeType.startsWith("audio/")}
    <FileAudio class="size-10 text-stone-600" />
  {:else if mimeType.startsWith("video/")}
    <FileVideo class="size-10 text-stone-600" />
  {:else if mimeType.startsWith("text/")}
    <FileText class="size-10 text-stone-600" />
  {:else}
    <File class="size-10 text-stone-600" />
  {/if}
{/snippet}

<form
  method="POST"
  class="flex flex-col"
  action="?/download"
  use:enhance={enhanceForm({
    messages: {
      failure: "Meow",
    },
    handlers: {
      success: async (result) => {
        const body = result.data?.body;
        try {
          const blob = new Blob([body], {
            type: "application/octet-stream",
          });
          const url = URL.createObjectURL(blob);
          const a = document.createElement("a");
          a.href = url;
          a.download = file.name;
          a.click();
          URL.revokeObjectURL(url);
          notification.set({
            message: `${file.name} downloaded successfully`,
            type: "success",
          });
        } catch (err) {
          notification.set({ message: "Failed to copy link", type: "error" });
        }
      },
    },
  })}
>
  <input type="hidden" value={file.s3Key} name="key" />
  <input type="hidden" value={file.name} name="filename" />
  <button
    type="submit"
    disabled={downloading}
    class="group relative flex aspect-square size-42 flex-col items-center justify-between rounded-md p-2 ring ring-stone-200 transition-colors hover:bg-stone-100 dark:ring-stone-800 dark:hover:bg-stone-800"
  >
    <H3>
      {file.name.split(".").shift()}
    </H3>

    <div class="">
      {@render icon(file.mimeType || "")}
    </div>
    <div
      class="flex items-center space-x-2 text-xs text-stone-500 dark:text-stone-400"
    >
      <span>{formatFileSize(file.size)}</span>
      {#if getFileExtension(file.name)}
        <span
          class="rounded bg-stone-100 px-1.5 py-0.5 text-xs dark:bg-stone-800"
        >
          {getFileExtension(file.name)}
        </span>
      {/if}
    </div>
    {#if $isLoading}
      <div
        class="absolute inset-0 flex items-center justify-center bg-white/80 dark:bg-stone-900/80"
      >
        <Loader2 class="size-16 animate-spin" />
      </div>
    {/if}
  </button>
</form>
