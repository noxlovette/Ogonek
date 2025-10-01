<script lang="ts">
  import { enhanceForm } from "$lib/utils";
  import { user } from "$lib/stores";
  import { enhance } from "$app/forms";
  import type { FileSmall } from "$lib/types";
  import { FileText, Image, FileAudio, File, X, FilePlay } from "lucide-svelte";
  import Caption1 from "../typography/Caption1.svelte";
  import { Merger, UniButton, VStack } from "../UI";
  import Divider from "../UI/toolbar/Divider.svelte";

  let { file }: { file: FileSmall } = $props();
  let downloading = $state(false);
</script>

{#snippet icon(mimeType: string)}
  {#if mimeType.startsWith("image/")}
    <Image class="size-10 text-stone-600" />
  {:else if mimeType.startsWith("audio/")}
    <FileAudio class="size-10 text-stone-600" />
  {:else if mimeType.startsWith("video/")}
    <FilePlay class="size-10 text-stone-600" />
  {:else if mimeType.startsWith("text/")}
    <FileText class="size-10 text-stone-600" />
  {:else}
    <File class="size-10 text-stone-600" />
  {/if}
{/snippet}

<VStack styling="items-center">
  <form
    method="POST"
    action="?/download"
    class="flex w-full"
    use:enhance={enhanceForm({
      handlers: {
        success: async (result) => {
          const url = result.data?.url;

          const iframe = document.createElement("iframe");
          iframe.style.display = "none";
          iframe.src = url;
          document.body.appendChild(iframe);

          await new Promise((resolve) => setTimeout(resolve, 5000));
          document.body.removeChild(iframe);
        },
      },
    })}
  >
    <input type="hidden" value={file.s3Key} name="key" />
    <button
      type="submit"
      data-cy="file-task-card"
      disabled={downloading}
      class="group ring-default bg-clickable relative flex w-full items-center justify-between gap-4 overflow-clip rounded-xl p-2 shadow-sm"
    >
      <Caption1>
        {file.name.split(".").shift()?.slice(0, 15)}
      </Caption1>

      {@render icon(file.mimeType || "")}
    </button>
  </form>
  <Divider></Divider>
  {#if file.ownerId === $user.id}
    <form
      method="POST"
      action="?/download"
      use:enhance={enhanceForm({
        messages: {
          success: "Файл удален",
        },
        shouldUpdate: true,
      })}
    >
      <input type="hidden" value={file.id} name="fileId" />
      <Merger>
        <UniButton
          content="Удалить файл"
          Icon={X}
          formaction="?/deleteFile"
          type="submit"
        ></UniButton>
      </Merger>
    </form>
  {/if}
</VStack>
