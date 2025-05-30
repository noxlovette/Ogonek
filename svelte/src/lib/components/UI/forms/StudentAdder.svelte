<script lang="ts">
  import { enhanceForm } from "$lib/utils";
  import { enhance } from "$app/forms";
  import { clickOutside } from "$lib/actions";
  import { notification } from "$lib/stores";
  import { PersonStanding, X } from "lucide-svelte";
  import UniButton from "../UniButton.svelte";
  import { Caption } from "$lib/components/typography";
  import H4 from "$lib/components/typography/H4.svelte";
  import Toggler from "../interactive/Toggler.svelte";

  let showPopover = false;
</script>

<div class="relative inline-block">
  <UniButton
    type="button"
    onclick={() => (showPopover = !showPopover)}
    variant="primary"
    Icon={PersonStanding}
  >
    Invite Students
  </UniButton>

  {#if showPopover}
    <div
      class="absolute right-0 z-50 mt-2 w-64 space-y-2 rounded-xl bg-white p-4 shadow-lg ring ring-stone-300/50 dark:bg-stone-900 "
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
              const link = String(result.data?.link);
              try {
                await navigator.clipboard.writeText(link);
                notification.set({
                  message: "Link copied!",
                  type: "success",
                });
                showPopover = false;
              } catch {
                notification.set({
                  message: "Copy failed",
                  type: "error",
                });
              }
            },
          },
        })}
      >
        <Caption>Do they have an account?</Caption>
        <Toggler name="isRegistered" title="Yes"></Toggler>
        <UniButton
          type="submit"
          size="sm"
          variant="primary"
          Icon={PersonStanding}
        >
          Generate Link
        </UniButton>
      </form>
    </div>
  {/if}
</div>
