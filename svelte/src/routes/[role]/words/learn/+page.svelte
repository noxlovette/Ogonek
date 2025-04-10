<script lang="ts">
  import { fade, slide } from "svelte/transition";
  import { quintOut } from "svelte/easing";
  import { enhance } from "$app/forms";
  import { invalidate } from "$app/navigation";
  import { enhanceForm } from "$lib/utils";
  import UniButton from "$lib/components/UI/UniButton.svelte";
  import { CheckCheck, Eye, Home } from "lucide-svelte";
  import { HeaderEmbellish } from "$lib/components";

  let { data } = $props();

  let currentIndex = $state(0);
  let currentCard = $derived(data.cards[currentIndex]);

  let isComplete = $state(data.cards.length === 0);
  let showAnswer = $state(false);
  $inspect(currentCard);
  const nextCard = async () => {
    if (currentIndex < data.cards.length - 1) {
      console.debug("not invalidated");
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
      label: "Blackout 💀",
      color: "ring-red-600 hover:bg-red-700/10 ring-2",
    },
    {
      quality: 1,
      label: "Wrong 🤔",
      color: "ring-orange-500 hover:bg-orange-600/10 ring-2",
    },
    {
      quality: 2,
      label: "Hard 😅",
      color: "ring-yellow-500 hover:bg-yellow-600/10 ring-2",
    },
    {
      quality: 3,
      label: "Okay 👍",
      color: "ring-stone-400 hover:bg-stone-400/10 ring-2",
    },
    {
      quality: 4,
      label: "Good 🎯",
      color: "ring-green-500 hover:bg-green-600/10 ring-2",
    },
    {
      quality: 5,
      label: "Easy ⚡",
      color: "ring-cacao-500 hover:bg-cacao-600/10 ring-2",
    },
  ];
</script>

{#if isComplete || data.cards.length === 0}
  <div class="p-8" in:fade={{ duration: 300 }}>
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
  <div class="space-y-6" in:fade={{ duration: 300 }}>
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
      class="flex min-h-[350px] flex-col rounded-lg bg-white p-8 shadow-sm ring ring-stone-200 transition-all hover:shadow-sm dark:bg-stone-900 dark:ring-stone-900"
      in:slide={{ duration: 300, easing: quintOut }}
    >
      <div class="flex-grow">
        <!-- Front side -->
        <div class="prose dark:prose-invert max-w-none">
          <h3
            class="mb-4 text-sm font-medium text-stone-500 uppercase dark:text-stone-400"
          >
            Question
          </h3>
          <div class="text-lg">{currentCard.front}</div>
        </div>

        <!-- Back side -->
        {#if showAnswer}
          <div
            class="mt-8 border-t border-stone-200 pt-8 dark:border-stone-700"
            transition:slide
          >
            <h3
              class="mb-4 text-sm font-medium text-stone-500 uppercase dark:text-stone-400"
            >
              Answer
            </h3>
            <div class="prose dark:prose-invert max-w-none">
              {currentCard.back}
            </div>
            {#if currentCard.mediaUrl}
              <img src={currentCard.mediaUrl} alt={currentCard.front} />
            {/if}
          </div>
        {/if}
      </div>

      <!-- Action buttons -->
      <div class="mt-8 flex justify-center">
        {#if !showAnswer}
          <UniButton
            Icon={Eye}
            onclick={() => (showAnswer = !showAnswer)}
            type="button"
            variant="outline"
          >
            Show Answer
          </UniButton>
        {:else}
          <form
            method="POST"
            class="grid w-full grid-cols-2 gap-2 sm:grid-cols-3 md:grid-cols-6"
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
