<script lang="ts">
  import { fade, slide } from "svelte/transition";
  import { quintOut } from "svelte/easing";
  import { enhance } from "$app/forms";
  import { notification } from "$lib/stores/notification.js";
  import { Anchor } from "$lib/components";
  import { invalidate } from "$app/navigation";
  import { shuffleArray } from "$lib/utils";

  let { data } = $props();

  let currentIndex = $state(0);
  let currentCard = $derived(shuffleArray(data.cards)[currentIndex]);

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
    {
      quality: 0,
      label: "Blackout 💀",
      color: "ring-red-600 hover:bg-red-700 ring-2",
    },
    {
      quality: 1,
      label: "Wrong 🤔",
      color: "ring-orange-500 hover:bg-orange-600 ring-2",
    },
    {
      quality: 2,
      label: "Hard 😅",
      color: "ring-yellow-500 hover:bg-yellow-600 ring-2",
    },
    {
      quality: 3,
      label: "Okay 👍",
      color: "ring-milk-100 hover:bg-milk-200 ring-2",
    },
    {
      quality: 4,
      label: "Good 🎯",
      color: "ring-green-500 hover:bg-green-600 ring-2",
    },
    {
      quality: 5,
      label: "Easy ⚡",
      color: "ring-cacao-500 hover:bg-cacao-600 ring-2",
    },
  ];

  function handleFormSuccess() {
    nextCard();
  }
</script>

{#if isComplete || data.cards.length === 0}
  <div class="p-8" in:fade={{ duration: 300 }}>
    <div class="flex flex-col items-center space-y-6 py-10 text-center">
      <div
        class="bg-cacao-50 dark:bg-cacao-900 flex h-20 w-20 items-center justify-center rounded-full"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="text-cacao-500 dark:text-cacao-400 h-12 w-12"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M5 13l4 4L19 7"
          />
        </svg>
      </div>
      <h2 class="text-2xl font-bold">🎉 All caught up!</h2>
      <p class="text-milk-600 dark:text-milk-400 max-w-md">
        You've reviewed all your due cards. Come back later for new cards to
        review.
      </p>
      <Anchor href=".">Return to the Words Page</Anchor>
    </div>
  </div>
{:else if currentCard}
  <div class="space-y-6" in:fade={{ duration: 300 }}>
    <!-- Progress bar with card count indication -->
    <div class="flex items-center justify-between">
      <span class="text-milk-600 dark:text-milk-400 text-sm">
        Card {data.cards.indexOf(currentCard) + 1} of {data.cards.length}
      </span>
      <span class="text-cacao-600 dark:text-cacao-400 text-sm font-medium">
        {Math.round(
          ((data.cards.indexOf(currentCard) + 1) / data.cards.length) * 100,
        )}% Complete
      </span>
    </div>

    <div
      class="bg-milk-200 dark:bg-milk-700 h-2.5 w-full overflow-hidden rounded-full"
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
      class="dark:bg-milk-900 flex min-h-[350px] flex-col rounded-xl bg-white p-8 shadow-sm transition-all hover:shadow-md"
      in:slide={{ duration: 300, easing: quintOut }}
    >
      <div class="flex-grow">
        <!-- Front side -->
        <div class="prose dark:prose-invert max-w-none">
          <h3
            class="text-milk-500 dark:text-milk-400 mb-4 text-sm font-medium uppercase"
          >
            Question
          </h3>
          <div class="text-lg">{currentCard.front}</div>
        </div>

        <!-- Back side -->
        {#if showAnswer}
          <div
            class="border-milk-200 dark:border-milk-700 mt-8 border-t pt-8"
            transition:slide
          >
            <h3
              class="text-milk-500 dark:text-milk-400 mb-4 text-sm font-medium uppercase"
            >
              Answer
            </h3>
            <div class="prose dark:prose-invert max-w-none">
              {currentCard.back}
            </div>
          </div>
        {/if}
      </div>

      <!-- Action buttons -->
      <div class="mt-8 flex justify-center">
        {#if !showAnswer}
          <button
            class="bg-cacao-600 hover:bg-cacao-700 text-cacao-50 flex items-center justify-center rounded-lg px-6 py-3 font-medium transition"
            onclick={() => (showAnswer = !showAnswer)}
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="mr-2 h-5 w-5"
              viewBox="0 0 20 20"
              fill="currentColor"
            >
              <path d="M10 12a2 2 0 100-4 2 2 0 000 4z" />
              <path
                fill-rule="evenodd"
                d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z"
                clip-rule="evenodd"
              />
            </svg>
            Show Answer
          </button>
        {:else}
          <form
            method="POST"
            class="grid w-full grid-cols-2 gap-2 sm:grid-cols-3 md:grid-cols-6"
            use:enhance={() => {
              return async ({ result }) => {
                if (result.type === "success") {
                  handleFormSuccess();
                } else if (result.type === "failure") {
                  notification.set({
                    message: String(result.data?.message) || "Something's Off",
                    type: "error",
                  });
                }
              };
            }}
          >
            <input type="hidden" bind:value={currentCard.id} name="cardId" />
            {#each qualityButtons as { quality, label, color }}
              <button
                class={`rounded-lg px-3 py-2 text-sm font-medium transition ${color}`}
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
  </div>
{/if}
