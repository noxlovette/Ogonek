<script lang="ts">
  import { fade, slide } from "svelte/transition";
  import { quintOut } from "svelte/easing";
  import { enhance } from "$app/forms";
  import { invalidate } from "$app/navigation";
  import { enhanceForm } from "$lib/utils";
  import { CheckCheck, Home } from "lucide-svelte";
  import { HeaderEmbellish, UniButton, Label, KBD } from "$lib/components";

  let { data } = $props();

  let currentIndex = $state(0);
  let currentCard = $derived(data.cards[currentIndex]);

  let isComplete = $state(data.cards.length === 0);
  let showAnswer = $state(false);
  const nextCard = async () => {
    if (currentIndex < data.cards.length - 1) {
      currentIndex++;
      showAnswer = false;
    } else if ((currentIndex = data.cards.length) && data.cards.length > 1) {
      invalidate("learn:complete");
      currentIndex = 0;
      showAnswer = false;
    } else {
      isComplete = true;
    }
  };

  const qualityButtons = [
    {
      quality: 0,
      label: "1066",
      color: "ring-red-600 hover:bg-red-700/10 ring",
      key: 1,
    },
    {
      quality: 1,
      label: "Wrong",
      color: "ring-orange-500 hover:bg-orange-600/10 ring",
      key: 2,
    },
    {
      quality: 2,
      label: "Hard",
      color: "ring-yellow-500 hover:bg-yellow-600/10 ring",
      key: 3,
    },
    {
      quality: 3,
      label: "Okay",
      color: "ring-stone-400 hover:bg-stone-400/10 ring",
      key: 4,
    },
    {
      quality: 4,
      label: "Good",
      color: "ring-green-500 hover:bg-green-600/10 ring",
      key: 5,
    },
    {
      quality: 5,
      label: "Easy",
      color: "ring-cacao-500 hover:bg-cacao-600/10 ring",
      key: 6,
    },
  ];

  function handleKeyPress(event: KeyboardEvent) {
    if (!showAnswer) {
      showAnswer = true;
    }

    const key = event.key;

    // Find the button with the matching data-key attribute
    const matchingButton: HTMLButtonElement | null = document.querySelector(
      `button[data-key="${key}"]`,
    );

    if (matchingButton) {
      event.preventDefault();
      matchingButton.click();
    }
  }
</script>

{#snippet qualityButton(quality: {
  key: number;
  quality: number;
  color: string;
  label: string;
})}
  <button
    class={`flex h-full flex-col items-center justify-center space-y-2 rounded-lg px-2 py-2 text-sm font-medium transition ${quality.color}`}
    name="quality"
    value={quality.quality}
    data-key={quality.key}
    type="submit"
  >
    <p>
      {quality.label}
    </p>
    <KBD>
      {quality.key}
    </KBD>
  </button>
{/snippet}

<svelte:window on:keydown={handleKeyPress} />
{#if isComplete || data.cards.length === 0}
  <div class="p-8" in:fade={{ duration: 100 }}>
    <div class="flex flex-col items-center space-y-6 py-10 text-center">
      <div
        class="bg-cacao-50 mx-auto flex h-16 w-16 items-center justify-center rounded-full dark:bg-stone-800"
      >
        <CheckCheck />
      </div>
      <h2 class="text-2xl font-bold">🎉 All caught up!</h2>
      <p class="max-w-md text-stone-600 dark:text-stone-400">
        You've reviewed all your due cards. Come back later for new cards to
        review.
      </p>

      <UniButton href="." Icon={Home} variant="primary"
        >Return to the Words page</UniButton
      >
    </div>
  </div>
{:else if currentCard}
  <div class="space-y-6" in:fade={{ duration: 100 }}>
    <HeaderEmbellish>
      <span class="text-sm text-stone-600 dark:text-stone-400">
        Card {data.cards.indexOf(currentCard) + 1} of {data.cards.length}
      </span>
      <span class="text-cacao-600 dark:text-cacao-400 text-sm font-medium">
        {Math.round(
          ((data.cards.indexOf(currentCard) + 1) / data.cards.length) * 100,
        )}% Complete
      </span>
    </HeaderEmbellish>
    <div
      class="h-2.5 w-full overflow-hidden rounded-full ring ring-stone-200 dark:bg-stone-700 dark:ring-stone-800"
    >
      <div
        class="bg-cacao-600 dark:bg-cacao-600 h-2.5 rounded-full transition-all duration-300"
        style="width: {((data.cards.indexOf(currentCard) + 1) /
          data.cards.length) *
          100}%"
      ></div>
    </div>

    <!-- Card container -->
    <div
      class="grid min-h-[350px] w-full gap-4 rounded-lg p-4 ring ring-stone-200 transition-all md:grid-cols-3 dark:ring-stone-900"
      in:slide={{ duration: 100, easing: quintOut }}
    >
      <div
        class="col-span-2 flex flex-col space-y-4 divide-y-2 divide-stone-200 dark:divide-stone-800"
      >
        <!-- Front side -->
        <div class="flex-grow">
          <Label>Question</Label>
          <div class="text-lg">{currentCard.front}</div>
        </div>
        <!-- Back side -->
        {#if showAnswer}
          <div class="flex-grow">
            <Label>Answer</Label>
            <p>
              {currentCard.back}
            </p>
            {#if currentCard.mediaUrl}
              <div class="mt-4 flex justify-center">
                <img
                  src={currentCard.mediaUrl}
                  alt={currentCard.front}
                  class="max-h-[200px] rounded-lg object-contain shadow-sm ring-1 ring-stone-200 dark:ring-stone-700"
                />
              </div>
            {/if}
          </div>
        {/if}
      </div>

      <!-- Action buttons container - always spans 1 column on md screens -->
      <div class="col-span-2 flex h-full md:col-span-1">
        {#if !showAnswer}
          <button
            class="flex w-full flex-col items-center justify-center space-y-2 rounded-lg bg-stone-50 ring ring-stone-200 transition-colors hover:bg-stone-100 dark:bg-stone-800 dark:ring-stone-700"
            onclick={() => (showAnswer = !showAnswer)}
          >
            <p>Flip</p>
            <KBD>Space</KBD>
          </button>
        {:else}
          <form
            method="POST"
            class="grid w-full grid-cols-3 gap-2 self-end md:h-full md:grid-cols-2"
            use:enhance={enhanceForm({
              messages: {
                failure: "Something's off",
              },
              handlers: {
                success: async () => {
                  nextCard();
                },
              },
            })}
          >
            <input type="hidden" value={currentCard.id} name="cardId" />
            {#each qualityButtons as quality}
              {@render qualityButton(quality)}
            {/each}
          </form>
        {/if}
      </div>
    </div>
  </div>
{/if}
