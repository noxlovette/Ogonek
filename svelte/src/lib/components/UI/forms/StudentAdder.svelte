<script lang="ts">
  import { enhanceForm } from "$lib/utils";
  import { enhance } from "$app/forms";
  import { clickOutside } from "$lib/actions";
  import { notification } from "$lib/stores";
  import { X, Copy, Plus } from "lucide-svelte";
  import UniButton from "../UniButton.svelte";
  import { Caption } from "$lib/components/typography";
  import H4 from "$lib/components/typography/H4.svelte";
  import Toggler from "../interactive/Toggler.svelte";

  let showPopover = false;
  let generatedLink = "";
  let linkInput: HTMLInputElement;

  async function copyLink() {
    try {
      await navigator.clipboard.writeText(generatedLink);
      notification.set({
        message: "Link copied!",
        type: "success",
      });
    } catch {
      linkInput?.select();
      notification.set({
        message: "Please copy manually",
        type: "info",
      });
    }
  }
</script>

<div class="relative inline-block">
  <UniButton
    type="button"
    onclick={() => (showPopover = !showPopover)}
    variant="primary"
    Icon={Plus}
  >
    Invite Students
  </UniButton>
  {#if showPopover}
    <div
      class="absolute right-0 z-50 mt-2 w-64 space-y-2 rounded-xl bg-white p-4 shadow-lg ring ring-stone-300/50 dark:bg-stone-900"
      use:clickOutside={() => (showPopover = false)}
    >
      <div class="flex items-center justify-between">
        <H4>Invite Students</H4>
        <button
          class="text-stone-400 hover:text-stone-700"
          type="button"
          onclick={() => (showPopover = false)}
        >
          <X size={16} />
        </button>
      </div>
      <form
        method="POST"
        class="space-y-2"
        use:enhance={enhanceForm({
          messages: { failure: "Failed to generate link" },
          handlers: {
            success: async (result) => {
              generatedLink = String(result.data?.link);
              notification.set({
                message: "Link generated!",
                type: "success",
              });
            },
          },
        })}
      >
        <Caption>Do they have an account?</Caption>
        <Toggler name="isRegistered" title="Yes"></Toggler>
        <UniButton type="submit" variant="primary" Icon={Plus}>
          Generate Link
        </UniButton>
      </form>

      {#if generatedLink}
        <div class="mt-3 space-y-1">
          <Caption>Invite Link:</Caption>
          <div class="flex items-center gap-1">
            <input
              bind:this={linkInput}
              readonly
              value={generatedLink}
              class="flex-1 rounded border border-stone-300 bg-stone-50 p-2 font-mono text-xs text-stone-700 dark:border-stone-600 dark:bg-stone-800 dark:text-stone-300"
            />
            <button
              type="button"
              onclick={copyLink}
              class="rounded p-2 text-stone-500 hover:bg-stone-100 hover:text-stone-700 dark:hover:bg-stone-700 dark:hover:text-stone-300"
              title="Copy link"
            >
              <Copy size={14} />
            </button>
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>
