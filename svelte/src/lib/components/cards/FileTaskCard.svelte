<script lang="ts">
  import { enhanceForm } from "$lib/utils";
  import { isLoading, user } from "$lib/stores";
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
    X,
  } from "lucide-svelte";
  import Label from "../typography/Label.svelte";

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

<div class="relative">
  {#if file.ownerId === $user.sub}
    <form
      action="?/deleteFile"
      method="POST"
      class="absolute top-1 right-1 z-50"
      use:enhance={enhanceForm({
        messages: {
          success: "Deleted File",
        },
      })}
    >
      <input type="hidden" value={file.id} name="fileId" />
      <button
        type="submit"
        class="text-stone-400 hover:text-red-500 dark:text-stone-300"
      >
        <X />
      </button>
    </form>
  {/if}
  <form
    method="POST"
    class="relative flex w-full flex-col"
    action="?/download"
    use:enhance={enhanceForm({
      messages: {
        failure: "Meow",
        success: `${file.name} download started`,
      },
      handlers: {
        success: async (result) => {
          const url = result.data?.url;

          const iframe = document.createElement("iframe");
          iframe.style.display = "none";
          iframe.src = url;
          document.body.appendChild(iframe);
          setTimeout(() => {
            document.body.removeChild(iframe);
          }, 5000); // Give it some time to initiate the download
        },
      },
    })}
  >
    <input type="hidden" value={file.s3Key} name="key" />
    <button
      type="submit"
      disabled={downloading}
      class="group relative flex h-40 w-full flex-col items-center justify-between overflow-clip rounded-md p-2 ring ring-stone-200 transition-colors hover:bg-stone-100 dark:ring-stone-800 dark:hover:bg-stone-800"
    >
      <Label>
        {file.name.split(".").shift()?.slice(0, 15)}
      </Label>

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
</div>
