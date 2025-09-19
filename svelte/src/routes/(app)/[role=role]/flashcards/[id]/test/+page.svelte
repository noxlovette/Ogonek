<script lang="ts">
  import { ArrowLeft, Ban, Check, Eye, RotateCcw } from "lucide-svelte";
  import {
    Body,
    Callout,
    Caption1,
    Caption2,
    Divider,
    HStack,
    LargeTitle,
    Merger,
    ProgressBar,
    KBD,
    Title2,
    Title3,
    Toolbar,
  } from "$lib/components";
  import { onMount } from "svelte";

  let { data } = $props();
  let { deck, cards } = data;
  let currentCardIndex = $state(0);
  let showAnswer = $state(false);
  let correctCount = $state(0);
  let totalAnswered = $state(0);
  let isTestComplete = $state(false);
  let startTime = Date.now();

  function revealAnswer() {
    showAnswer = true;
  }

  function markAnswer(isCorrect: boolean) {
    totalAnswered++;
    if (isCorrect) correctCount++;

    if (currentCardIndex < cards.length - 1) {
      currentCardIndex++;
      showAnswer = false;
    } else {
      isTestComplete = true;
    }
  }

  onMount(() => {
    startTime = Date.now();
  });

  let timeElapsed = $state(0);

  const currentCard = $derived(cards[currentCardIndex]);
  const progress = $derived(
    cards.length > 0 ? ((currentCardIndex + 1) / cards.length) * 100 : 0,
  );

  const accuracy = $derived(
    totalAnswered > 0 ? Math.round((correctCount / totalAnswered) * 100) : 0,
  );
  $effect(() => {
    setInterval(() => {
      timeElapsed = Math.floor((Date.now() - startTime) / 1000);
    }, 1000);
  });

  function handleKeyPress(event: KeyboardEvent) {
    if (!showAnswer && event.key == " ") {
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
</script>

<svelte:head>
  <title>Test Mode • {deck.title}</title>
</svelte:head>

<svelte:window on:keydown={handleKeyPress} />
<Toolbar>
  <HStack>
    <LargeTitle>Test Mode</LargeTitle>
    <div class="flex-1 items-end justify-end">
      <HStack>
        <ProgressBar {progress} />
        <HStack styling="gap-1">
          <Caption1>Card {currentCardIndex + 1} of {cards.length}</Caption1>
          <Caption2
            >Time elapsed {Math.floor(timeElapsed / 60)}:{(timeElapsed % 60)
              .toString()
              .padStart(2, "0")}</Caption2
          >
        </HStack>
      </HStack>
    </div></HStack
  >
</Toolbar>

{#if isTestComplete}
  <HStack>
    <Title3>Score</Title3>
    <Body>
      {correctCount}/{cards.length}
    </Body>
    <Callout>
      {accuracy}% correct
    </Callout>
    <Divider></Divider>
    <Title3>Time</Title3>
    <Callout>
      {Math.floor(timeElapsed / 60)}:{(timeElapsed % 60)
        .toString()
        .padStart(2, "0")}
    </Callout>
  </HStack>
{:else}
  <div class="grid grid-cols-1 md:grid-cols-2">
    <Title2>
      {currentCard.front}
    </Title2>

    {#if currentCard.mediaUrl}
      <img
        src={currentCard.mediaUrl}
        alt="Card media"
        class="h-auto max-w-full rounded-lg"
      />
    {/if}

    {#if !showAnswer}
      <button
        class="ring-default bg-default flex w-full flex-col items-center justify-center space-y-2 rounded-lg p-2 hover:bg-stone-100 dark:hover:bg-stone-900"
        onclick={() => (showAnswer = !showAnswer)}
      >
        <p>Flip</p>
        <KBD>Space</KBD>
      </button>
    {:else}
      <HStack>
        <Title3>
          {currentCard.back}
        </Title3>

        <button
          class="flex h-full flex-col items-center justify-center gap-2 rounded-lg bg-red-500/20 p-2 font-medium transition"
          data-key={1}
          type="button"
          onclick={() => markAnswer(false)}
        >
          <Callout>Wrong</Callout>
          <KBD>1</KBD>
        </button>
        <button
          class="flex h-full flex-col items-center justify-center gap-2 rounded-lg bg-green-500/20 p-2 font-medium transition"
          data-key={2}
          type="button"
          onclick={() => markAnswer(true)}
        >
          <Callout>Correct</Callout>
          <KBD>2</KBD>
        </button>
      </HStack>
    {/if}
  </div>
{/if}
