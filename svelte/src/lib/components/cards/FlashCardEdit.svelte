<script lang="ts">
  import { fade } from "svelte/transition";
  import { Trash2, Image } from "lucide-svelte";
  import type { Card } from "$lib/types";
  import { Input } from "../UI";

  let {
    index,
    card,
    removeCard,
  }: { card: Card; index: number; removeCard: (index: number) => void } =
    $props();

  let showMediaInput = $state(!!card.mediaUrl);
</script>

<div
  class="group ring-default bg-default relative rounded-lg p-3 shadow-sm hover:ring-stone-300 dark:bg-stone-900 dark:hover:ring-stone-600"
>
  <button
    type="button"
    onclick={() => removeCard(index)}
    class="absolute -top-2 -right-2 hidden rounded-full bg-stone-500 p-1.5 text-stone-50 hover:bg-red-700 focus:ring focus:ring-red-400 md:block"
    title="Remove card"
  >
    <Trash2 class="size-4" />
  </button>

  <!-- Card inputs in a compact grid -->
  <div class="grid gap-4 md:grid-cols-2">
    <!-- Front -->
    <div>
      <Input
        type="textarea"
        labelName="Лицевая сторона"
        name={`cards[${index}][front]`}
        bind:value={card.front}
        placeholder="Question or prompt"
      />
    </div>

    <!-- Back -->
    <div>
      <Input
        labelName="Обратная сторона"
        type="textarea"
        name={`cards[${index}][back]`}
        bind:value={card.back}
        placeholder="Answer or explanation"
      />
    </div>
  </div>

  <button
    type="button"
    onclick={() => (showMediaInput = !showMediaInput)}
    class="mt-3 flex items-center gap-1.5 text-sm text-stone-600 hover:text-stone-900 dark:text-stone-400 dark:hover:text-stone-200"
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
        bind:value={card.mediaUrl}
        placeholder="https://example.com/image.jpg"
        class="focus:ring-accent w-full rounded-md bg-transparent px-3 py-1.5 text-sm ring ring-stone-300 placeholder:text-stone-400 focus:ring-1 focus:outline-none dark:ring-stone-600 dark:placeholder:text-stone-500"
      />
    </div>
  {/if}
</div>
