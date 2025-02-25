<script lang="ts">
  import { fade } from "svelte/transition";
  import { Trash2, Image } from "lucide-svelte";
  import type { Card } from "$lib/types";
  import { Label } from "../typography";
  import { Input } from "../UI";

  let {
    index,
    card,
    removeCard,
  }: { card: Card; index: number; removeCard: (index: number) => void } =
    $props();

  let showMediaInput = $state(!!card.media_url);
</script>

<div
  class="group border-milk-200 hover:border-milk-300 dark:border-milk-800 dark:bg-milk-900 dark:hover:border-milk-600 relative rounded-lg border bg-white p-3 shadow-sm transition-all"
  transition:fade
>
  <button
    type="button"
    onclick={() => removeCard(index)}
    class="absolute -top-2 -right-2 hidden rounded-full bg-red-500/90 p-1.5 text-white opacity-0 transition-all group-hover:opacity-100 hover:bg-red-600 focus:ring focus:ring-red-400 md:block"
    title="Remove card"
  >
    <Trash2 class="h-4 w-4" />
  </button>

  <!-- Card inputs in a compact grid -->
  <div class="grid gap-3 md:grid-cols-2">
    <!-- Front -->
    <div class="space-y-1.5">
      <Label>Front</Label>
      <Input
        type="textarea"
        name={`cards[${index}][front]`}
        bind:value={card.front}
        placeholder="Question or prompt"
      />
    </div>

    <!-- Back -->
    <div class="space-y-1.5">
      <Label>Back</Label>
      <Input
        type="textarea"
        name={`cards[${index}][back]`}
        bind:value={card.back}
        placeholder="Answer or explanation"
      />
    </div>
  </div>

  <!-- Toggle media input -->
  <button
    type="button"
    onclick={() => (showMediaInput = !showMediaInput)}
    class="text-milk-600 hover:text-milk-900 dark:text-milk-400 dark:hover:text-milk-200 mt-3 flex items-center gap-1.5 text-sm"
  >
    <Image class="h-4 w-4" />
    {showMediaInput ? "Hide media URL" : "Add media"}
  </button>

  <!-- Media URL input - shows only when needed -->
  {#if showMediaInput}
    <div class="mt-2">
      <input
        type="url"
        name={`cards[${index}][media_url]`}
        bind:value={card.media_url}
        placeholder="https://example.com/image.jpg"
        class="border-milk-300 placeholder:text-milk-400 focus:border-cocoa-500 focus:ring-cocoa-500 dark:border-milk-600 dark:placeholder:text-milk-500 w-full rounded-md border bg-transparent px-3 py-1.5 text-sm focus:ring-1 focus:outline-none"
      />
    </div>
  {/if}
</div>
