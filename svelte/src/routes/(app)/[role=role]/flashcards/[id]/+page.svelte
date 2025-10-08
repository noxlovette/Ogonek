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
  import { Bookmark, BookOpenCheck, Copy } from "@lucide/svelte";
  import { enhanceForm } from "$lib/utils";
  import Badge from "$lib/components/cards/Badge.svelte";
  import { page } from "$app/state";
  import texts from "$lib/texts.js";

  let { data } = $props();

  const { deck, cards } = data;
  let flippedCards = $state(new Set<string>());

  let isSubscribed = $state(data.deck.isSubscribed);

  const assigneeName = $derived(
    data.students.find((student) => student.id === data.deck.assignee)?.name ||
      deck.visibility,
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
  <title>{deck.title} | Карточки</title>
</svelte:head>
<Toolbar>
  <HStack>
    <VStack>
      <LargeTitle>{deck.title}</LargeTitle>
      <Divider></Divider>
      <VStack>
        <Merger>
          <UniButton
            content="Режим тренировки"
            href="{deck.id}/test"
            Icon={BookOpenCheck}
          ></UniButton>

          <form
            method="POST"
            action="?/subscribe"
            use:enhance={enhanceForm({
              messages: {
                success: isSubscribed ? "Вы отписались" : "Вы подписались",
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
              Icon={Bookmark}
              fill={isSubscribed ? true : false}
              type="submit"
              variant={page.params.role === "t" ? "primary" : "prominent"}
              content={isSubscribed ? "Отписаться" : "Подписаться"}
            ></UniButton>
          </form>
        </Merger>
        <Merger>
          <form
            method="POST"
            action="?/duplicate"
            use:enhance={enhanceForm({
              messages: {
                success: "Дубликат создан",
              },
            })}
          >
            <UniButton Icon={Copy} content="Создать дубликат" type="submit"
            ></UniButton>
          </form>
          {#if $user.id === deck.createdBy}
            <EditButton href="{deck.id}/edit" />
          {/if}
        </Merger>
      </VStack>
    </VStack>
    <VStack>
      {#if deck.description && deck.description?.trim() !== ""}
        <div class="flex flex-wrap gap-x-1 gap-y-2">
          {#each deck.description.split(";") as deckTag, index (index)}
            <Badge>{deckTag}</Badge>
          {/each}
        </div>
      {/if}
      <Badge>
        {cards.length}
      </Badge>
      {#if page.params.role == "t"}
        <Badge>
          {assigneeName}
        </Badge>
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
    {texts.table.empty}
  </EmptySpace>
{/if}
