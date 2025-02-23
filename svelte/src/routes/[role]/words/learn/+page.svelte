<script lang="ts">
  import { page } from "$app/state";
  import { goto } from "$app/navigation";
  import type { CardProgress } from "$lib/types";
  import { fade, slide } from "svelte/transition";
  import { quintOut } from "svelte/easing";
  import { enhance } from "$app/forms";
  import { notification } from "$lib/stores/notification.js";

  let { data } = $props();

  let currentCard = $derived(
    data.cards.find((c) => c.id === page.url.searchParams.get("cardId")) ||
      data.cards[0],
  );

  let showAnswer = $state(false);

  const qualityButtons = [
    { quality: 0, label: "Blackout 💀" },
    { quality: 1, label: "Wrong 🤔" },
    { quality: 2, label: "Hard 😅" },
    { quality: 3, label: "Okay 👍" },
    { quality: 4, label: "Good 🎯" },
    { quality: 5, label: "Easy ⚡" },
  ];

  function getNextCard(currentCardId: string): CardProgress | undefined {
    const currentIndex = data.cards.findIndex((c) => c.id === currentCardId);
    return data.cards[currentIndex + 1];
  }

  function handleFormSuccess() {
    const nextCard = getNextCard(currentCard.id);
    if (nextCard) {
      const params = new URLSearchParams();
      params.set("cardId", nextCard.id);
      goto(`?${params.toString()}`);
    } else {
      goto("");
    }
  }
</script>

<div class="mx-auto w-full max-w-4xl p-6">
  {#if data.cards.length === 0}
    <div class="py-12 text-center" in:fade>
      <h2 class=" mb-4 text-2xl font-bold">🎉 All caught up!</h2>
      <p class="">No cards due for review right now.</p>
    </div>
  {:else if currentCard}
    <div class="space-y-6" in:fade>
      <!-- Progress bar -->
      <div class="bg-milk-200 h-2.5 w-full rounded-full">
        <div
          class="bg-cocoa-600 h-2.5 rounded-full transition-all duration-300"
          style="width: {((data.cards.indexOf(currentCard) + 1) /
            data.cards.length) *
            100}%"
        ></div>
      </div>

      <div
        class="flex min-h-[300px] flex-col rounded-xl bg-white p-8 shadow-lg"
        in:slide={{ duration: 300, easing: quintOut }}
      >
        <div class="flex-grow">
          <!-- Front -->
          <div class="prose max-w-none">
            {currentCard.front}
          </div>

          <!-- Back -->
          {#if showAnswer}
            <div
              class="border-milk-200 prose mt-8 max-w-none border-t pt-8"
              transition:slide
            >
              {currentCard.back}
            </div>
          {/if}
        </div>

        <div class="mt-8">
          {#if !showAnswer}
            <button
              class="bg-cocoa-600 hover:bg-cocoa-700 w-full rounded-lg px-6 py-3 text-white transition"
              onclick={() => (showAnswer = !showAnswer)}
            >
              Show Answer
            </button>
          {:else}
            <form
              method="POST"
              class="flex flex-wrap justify-center gap-2"
              use:enhance={() => {
                return async ({ result }) => {
                  if (result.type === "success") {
                    handleFormSuccess();
                  } else if (result.type === "failure") {
                    notification.set({
                      message: result.data?.message || "Something's Off",
                      type: "error",
                    });
                  }
                };
              }}
            >
              {#each qualityButtons as { quality, label }}
                <button
                  class="bg-milk-100 hover:bg-milk-200 min-w-[150px] flex-1 rounded-lg px-4 py-2 transition"
                  name="quality"
                  value={quality}
                  type="submit"
                >
                  {label}
                </button>
              {/each}
            </form>
          {/if}
        </div>
      </div>

      <!-- Stats -->
      <div class=" text-center">
        Card {data.cards.indexOf(currentCard) + 1} of {data.cards.length}
      </div>
    </div>
  {/if}
</div>
