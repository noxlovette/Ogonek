<script lang="ts">
  import { enhance } from "$app/forms";
  import { invalidate } from "$app/navigation";
  import { enhanceForm, qualityButtons } from "$lib/utils";
  import { GraduationCap } from "@lucide/svelte";
  import {
    Toolbar,
    UniButton,
    Caption1,
    KBD,
    Divider,
    ProgressBar,
    LargeTitle,
    Title3,
    Input,
    Headline,
    Subheadline,
    Body,
    Merger,
  } from "$lib/components";
  import Title1 from "$lib/components/typography/Title1.svelte";

  let { data } = $props();

  let currentIndex = $state(0);
  let isReloading = $state(false);
  let currentCard = $derived.by(() => {
    if (isReloading) return null;
    if (currentIndex >= data.cards.length) {
      return data.cards[0] || null;
    }
    return data.cards[currentIndex] || null;
  });

  let isComplete = $state(data.cards.length === 0);
  let showAnswer = $state(false);
  let showCloze = $derived(
    currentCard ? currentCard.front.split(/\s+/).length < 4 : false,
  );
  let userInput = $state("");

  const nextCard = async () => {
    userInput = "";
    showAnswer = false;

    if (currentIndex < data.cards.length - 1) {
      // Move to next card in current batch
      currentIndex++;
    } else {
      // We're on the last card - reload data to get new cards
      isReloading = true;
      await invalidate("learn:complete");
      currentIndex = 0;
      isReloading = false;
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

  const progress = $derived(
    currentCard && data.cards.length > 0
      ? Math.round(((currentIndex + 1) / data.cards.length) * 100)
      : 0,
  );

  let inputRef = $state<HTMLInputElement>();
  $effect(() => {
    if (showCloze && !showAnswer && inputRef) {
      const interval = setInterval(() => {
        if (document.activeElement !== inputRef) {
          if (inputRef) {
            inputRef.focus();
          }
        }
      }, 50);

      return () => clearInterval(interval);
    }
  });
</script>

{#snippet qualityButton(quality: {
  key: number;
  quality: number;
  color: string;
  label: string;
})}
  <button
    class={`flex h-full items-center justify-center gap-2 rounded-lg p-2  font-medium transition ${quality.color}`}
    name="quality"
    value={quality.quality}
    data-cy="quality-button"
    data-key={quality.key}
    type="submit"
    ><KBD>
      {quality.key}
    </KBD>
    <Subheadline>
      {quality.label}
    </Subheadline>
  </button>
{/snippet}

<svelte:window on:keydown={handleKeyPress} />
{#if isComplete || !data.cards.length}
  <div class="p-8">
    <div class="gap-default flex flex-col items-center py-10 text-center">
      <Title1>Карточек тут больше нет. Приходите завтра...</Title1>

      <Merger>
        <UniButton
          href="."
          Icon={GraduationCap}
          content="К декам"
          variant="primary"
        ></UniButton>
      </Merger>
    </div>
  </div>
{:else if currentCard}
  <Toolbar>
    <LargeTitle>Режим учёбы</LargeTitle>
    <Divider />
    <Title3>
      {data.cards.indexOf(currentCard) + 1} / {data.cards.length}
    </Title3>
  </Toolbar>
  <div
    class="ring-default h-2.5 w-full overflow-hidden rounded-full dark:bg-stone-700"
  >
    <ProgressBar {progress} />
  </div>

  <div class="grid min-h-[350px] gap-4 md:grid-cols-3">
    <div
      class="col-span-2 flex flex-col space-y-4 divide-y-2 divide-stone-200 dark:divide-stone-800"
    >
      <!-- Front side -->
      <div class="flex-grow">
        <Caption1>{showCloze ? "" : "Лицо"}</Caption1>

        {#if showCloze}
          {#if !showAnswer}
            <Input
              dataCy="answer-input"
              bind:ref={inputRef}
              name="Лицо"
              placeholder="Apple (n)"
              bind:value={userInput}
            />
          {:else}
            <div class="space-y-2">
              <Caption1>Ваш вариант</Caption1>
              <div class="rounded-lg bg-stone-100 p-3 dark:bg-stone-800">
                <Subheadline>{userInput}</Subheadline>
              </div>

              <Caption1>Оборот</Caption1>
              <div class="rounded-lg bg-emerald-100 p-3 dark:bg-emerald-900">
                <Title3>{currentCard.front}</Title3>
              </div>
            </div>
          {/if}
        {:else}
          <Headline>
            {currentCard.front}
          </Headline>
        {/if}
      </div>
      <!-- Back side -->
      {#if showCloze}
        <div class="flex-grow">
          <Caption1>Оборот</Caption1>
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
          <Caption1>Оборот</Caption1>
          <Title3>
            {currentCard.back}
          </Title3>
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
          class="grid w-full gap-2 self-end md:h-full"
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
