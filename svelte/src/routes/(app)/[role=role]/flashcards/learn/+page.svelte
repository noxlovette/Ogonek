<script lang="ts">
  import { fade, slide } from "svelte/transition";
  import { quintOut } from "svelte/easing";
  import { enhance } from "$app/forms";
  import { invalidate } from "$app/navigation";
  import { enhanceForm, qualityButtons } from "$lib/utils";
  import { CheckCheck, GraduationCap } from "lucide-svelte";
  import {
    Toolbar,
    UniButton,
    Caption1,
    KBD,
    Callout,
    Divider,
    ProgressBar,
  } from "$lib/components";
  import { m } from "$lib/paraglide/messages";

  let { data } = $props();

  let currentIndex = $state(0);
  let currentCard = $derived(data.cards[currentIndex]);

  let isComplete = $state(data.cards.length === 0);
  let showAnswer = $state(false);
  let showCloze = $derived(currentCard.front.split(/\s+/).length < 4);
  let userInput = $state("");

  const nextCard = async () => {
    userInput = "";
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

  function handleKeyPress(event: KeyboardEvent) {
    if (!showAnswer && !showCloze && event.key == " ") {
      event.preventDefault();
      showAnswer = true;
    } else if (!showAnswer && showCloze && event.key == "Enter") {
      event.preventDefault();
      showAnswer = true;
    }

    const key = event.key;

    const matchingButton: HTMLButtonElement | null = document.querySelector(
      `button[data-key="${key}"]`,
    );

    if (matchingButton) {
      event.preventDefault();
      matchingButton.click();
    }
  }

  let ref = $state<HTMLInputElement>();

  $effect(() => {
    if (ref) {
      ref.focus();
    }
  });

  const progress = $derived(
    ((data.cards.indexOf(currentCard) + 1) / data.cards.length) * 100,
  );
</script>

{#snippet qualityButton(quality: {
  key: number;
  quality: number;
  color: string;
  label: string;
})}
  <button
    class={`flex h-full flex-col items-center justify-center gap-2 rounded-lg p-2 font-medium transition ${quality.color}`}
    name="quality"
    value={quality.quality}
    data-key={quality.key}
    type="submit"
  >
    <Callout>
      {quality.label}
    </Callout>
    <KBD>
      {quality.key}
    </KBD>
  </button>
{/snippet}

<svelte:window on:keydown={handleKeyPress} />
{#if isComplete || data.cards.length === 0}
  <div class="p-8">
    <div class="flex flex-col items-center space-y-6 py-10 text-center">
      <div
        class="bg-accent mx-auto flex h-16 w-16 items-center justify-center rounded-full dark:bg-stone-800"
      >
        <CheckCheck />
      </div>
      <p class="max-w-md text-stone-600 dark:text-stone-400">
        {m.quiet_lost_whale_exhale()}
      </p>

      <UniButton
        href="."
        Icon={GraduationCap}
        variant="primary"
        iconOnly={false}>{m.decks()}</UniButton
      >
    </div>
  </div>
{:else if currentCard}
  <Toolbar>
    <span class="text-sm text-stone-600 dark:text-stone-400">
      {data.cards.indexOf(currentCard) + 1} / {data.cards.length}
    </span>
    <Divider />
    <span class="text-accent dark:text-accent text-sm font-medium">
      {progress}% {m.complete()}
    </span>
  </Toolbar>
  <div
    class="ring-default h-2.5 w-full overflow-hidden rounded-full dark:bg-stone-700"
  >
    <ProgressBar {progress} />
  </div>

  <!-- Card container -->
  <div
    class="ring-default bg-default grid min-h-[350px] w-full gap-4 rounded-lg p-4 md:grid-cols-3"
    in:slide={{ duration: 100, easing: quintOut }}
  >
    <div
      class="col-span-2 flex flex-col space-y-4 divide-y-2 divide-stone-200 dark:divide-stone-800"
    >
      <!-- Front side -->
      <div class="flex-grow">
        <Caption1>{showCloze ? "" : m.cardFront()}</Caption1>

        {#if showCloze}
          {#if !showAnswer}
            <input
              bind:this={ref}
              bind:value={userInput}
              class="focus:border-accent focus:ring-accent w-full rounded-2xl border border-stone-100/60 bg-white px-4 py-2 text-base text-stone-900 placeholder-stone-400 shadow-sm focus:shadow-md focus:ring-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-60 dark:border-stone-800/60 dark:bg-stone-950 dark:text-stone-100"
              placeholder="Type in your answer..."
            />
          {:else}
            <div class="space-y-2">
              <Caption1>{m.cardFrontClozeUserInput()}</Caption1>
              <div class="rounded-lg bg-stone-100 p-3 dark:bg-stone-800">
                <span class="">{userInput}</span>
              </div>

              <Caption1>{m.cardFrontClozeCorrect()}</Caption1>
              <div class="rounded-lg bg-green-100 p-3 dark:bg-green-900">
                <span class="">{currentCard.front}</span>
              </div>
            </div>
          {/if}
        {:else}
          <div class="text-lg">{currentCard.front}</div>
        {/if}
      </div>
      <!-- Back side -->
      {#if showCloze}
        <div class="flex-grow">
          <Caption1>Question</Caption1>
          <p>
            {currentCard.back}
          </p>
          {#if currentCard.mediaUrl}
            <div class="mt-4 flex justify-center">
              <img
                src={currentCard.mediaUrl}
                alt={currentCard.front}
                class="max-h-[200px] rounded-lg object-contain shadow-sm ring-1 ring-stone-300/40"
              />
            </div>
          {/if}
        </div>
      {:else if showAnswer}
        <div class="flex-grow">
          <Caption1>{m.cardBack()}</Caption1>
          <p>
            {currentCard.back}
          </p>
          {#if currentCard.mediaUrl}
            <div class="mt-4 flex justify-center">
              <img
                src={currentCard.mediaUrl}
                alt={currentCard.front}
                class="max-h-[200px] rounded-lg object-contain shadow-sm ring-1 ring-stone-300/40"
              />
            </div>
          {/if}
        </div>
      {/if}
    </div>

    <div class="col-span-2 flex h-full md:col-span-1">
      {#if !showAnswer}
        <button
          class="ring-default bg-default flex w-full flex-col items-center justify-center space-y-2 rounded-lg hover:bg-stone-100 dark:hover:bg-stone-900"
          onclick={() => (showAnswer = !showAnswer)}
        >
          <p>Flip</p>
          <KBD>{showCloze ? "Enter" : "Space"}</KBD>
        </button>
      {:else}
        <form
          method="POST"
          class="grid w-full grid-cols-3 gap-2 self-end md:h-full md:grid-cols-2"
          use:enhance={enhanceForm({
            handlers: {
              success: async () => {
                nextCard();
              },
            },
          })}
        >
          <input type="hidden" value={currentCard.id} name="cardId" />
          {#each qualityButtons as quality (quality.key)}
            {@render qualityButton(quality)}
          {/each}
        </form>
      {/if}
    </div>
  </div>
{/if}
