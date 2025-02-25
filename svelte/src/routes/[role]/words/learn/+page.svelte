<script lang="ts">
  import { fade, slide } from "svelte/transition";
  import { quintOut } from "svelte/easing";
  import { enhance } from "$app/forms";
  import { notification } from "$lib/stores/notification.js";
  import { Anchor } from "$lib/components";
  import { invalidate } from "$app/navigation";

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

  $inspect(data.cards);

  const qualityButtons = [
    { quality: 0, label: "Blackout 💀" },
    { quality: 1, label: "Wrong 🤔" },
    { quality: 2, label: "Hard 😅" },
    { quality: 3, label: "Okay 👍" },
    { quality: 4, label: "Good 🎯" },
    { quality: 5, label: "Easy ⚡" },
  ];

  function handleFormSuccess() {
    nextCard();
  }
</script>

<div class="mx-auto w-full max-w-4xl p-6">
  {#if isComplete || data.cards.length === 0}
    <div class="space-y-4 py-12 text-center" in:fade>
      <h2 class=" mb-4 text-2xl font-bold">🎉 All caught up!</h2>
      <p class="">No cards due for review right now.</p>

      <Anchor href=".">Return to the Words Page</Anchor>
    </div>
  {:else if currentCard}
    <div class="space-y-6" in:fade>
      <!-- Progress bar -->
      <div class="bg-milk-200 h-2.5 w-full rounded-full">
        <div
          class="h-2.5 rounded-full bg-green-500 transition-all duration-300"
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
              class="bg-cacao-600 text-cacao-50 hover:bg-cacao-700 rounded-lg px-6 py-3 transition"
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
                      message:
                        String(result.data?.message) || "Something's Off",
                      type: "error",
                    });
                  }
                };
              }}
            >
              <input type="hidden" bind:value={currentCard.id} name="cardId" />
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
