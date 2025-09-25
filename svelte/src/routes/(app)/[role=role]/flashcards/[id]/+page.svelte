<script lang="ts">
  import { enhance } from "$app/forms";
  import {
    WordCard,
    LargeTitle,
    UniButton,
    Toolbar,
    EmptySpace,
    VStack,
    Merger,
    Caption1,
    HStack,
    Divider,
    EditButton,
  } from "$lib/components";
  import { invalidate } from "$app/navigation";

  import { user } from "$lib/stores";
  import { BookOpenCheck, Check, Circle, Copy } from "lucide-svelte";
  import { enhanceForm } from "$lib/utils";
  import Badge from "$lib/components/cards/Badge.svelte";
  import { page } from "$app/state";
  import { m } from "$lib/paraglide/messages";

  let { data } = $props();

  const { deck, cards } = data;
  let flippedCards = $state(new Set<string>());

  let isSubscribed = $state(data.deck.isSubscribed);

  const assigneeName = $derived(
    data.students.find((student) => student.id === data.deck.assignee)?.name ||
      m.notAssigned(),
  );

  const toggleCard = (cardId: string) => {
    flippedCards = new Set(
      flippedCards.has(cardId)
        ? [...flippedCards].filter((id) => id !== cardId)
        : [...flippedCards, cardId],
    );
  };
</script>

<svelte:head>
  <title>{deck.title} | Flashcards</title>
</svelte:head>
<Toolbar>
  <HStack>
    <VStack>
      <LargeTitle>{deck.title}</LargeTitle>
      <Divider></Divider>
      <VStack>
        <Merger>
          <UniButton
            content={m.knotty_ideal_marten_zip()}
            href="{deck.id}/test"
            Icon={BookOpenCheck}
          ></UniButton>
        </Merger>
        <Merger>
          <form
            method="POST"
            action="?/duplicate"
            use:enhance={enhanceForm({
              messages: {
                success: "Deck Duplicated",
              },
            })}
          >
            <UniButton
              Icon={Copy}
              content={m.less_calm_blackbird_play()}
              type="submit"
            ></UniButton>
          </form>
          {#if $user.id === deck.createdBy}
            <EditButton href="{deck.id}/edit" />
          {/if}
        </Merger>
        <Merger>
          <form
            method="POST"
            action="?/subscribe"
            use:enhance={enhanceForm({
              messages: {
                success: isSubscribed
                  ? m.elegant_small_gadfly_quell()
                  : m.stout_royal_macaw_fear(),
              },
              handlers: {
                success: async () => {
                  isSubscribed = !isSubscribed;
                  invalidate("learn:subscribe");
                },
              },
            })}
          >
            <input type="hidden" name="isSubscribed" value={isSubscribed} />
            <UniButton
              Icon={isSubscribed === true ? Check : Circle}
              type="submit"
              variant="prominent"
              content={isSubscribed
                ? m.fluffy_elegant_coyote_assure()
                : m.fit_least_baboon_imagine()}
            ></UniButton>
          </form>
        </Merger>
      </VStack>
    </VStack>
    <VStack>
      {#if deck.description}
        <div class="flex flex-wrap gap-x-1 gap-y-2">
          {#each deck.description.split(";") as deckTag, index (index)}
            <Badge>{deckTag}</Badge>
          {/each}
        </div>
      {/if}
      <Badge>
        {cards.length}
        {m.some_happy_cat({ count: cards.length })}
      </Badge>
      {#if page.params.role == "t"}
        <Caption1>
          {assigneeName}
        </Caption1>
      {/if}
    </VStack>
  </HStack>
</Toolbar>

<div class="grid gap-4 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
  {#each cards as card, _}
    <WordCard bind:flippedCards {card} {toggleCard} />
  {/each}
</div>

{#if cards.length === 0}
  <EmptySpace>
    <p class="text-lg text-stone-500 dark:text-stone-400">
      {m.noFlashcards()}
    </p>
    <p class="mt-2 text-sm text-stone-400 dark:text-stone-500">
      {m.deft_fuzzy_stingray_push()}
    </p>
  </EmptySpace>
{/if}
