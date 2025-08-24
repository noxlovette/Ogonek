<script lang="ts">
  import {
    Input,
    LargeTitle,
    FlashCardEdit,
    CSV,
    Toolbar,
    Divider,
    VStack,
    Merger,
    HStack,
    EmptySpace,
  } from "$lib/components";

  import { enhanceForm } from "$lib/utils";
  import { enhance } from "$app/forms";
  import { page } from "$app/state";
  import { Plus, Ban, Check, Trash2, Import } from "lucide-svelte";
  import UniButton from "$lib/components/UI/UniButton.svelte";
  import { pushState } from "$app/navigation";
  import { onMount } from "svelte";
  import { m } from "$lib/paraglide/messages.js";
  import Title1 from "$lib/components/typography/Title1.svelte";

  onMount(() => {
    if (updatedCards.length > 0) {
      const lastCard = document.querySelector(".space-y-4 > :last-child");
      if (lastCard) {
        lastCard.scrollIntoView({ behavior: "smooth" });
      }
    }
  });
  function showImportModal() {
    pushState("import", {
      showImportModal: true,
    });
  }

  const role = page.params.role;
  let { data } = $props();
  let { deck, cards } = data;
  let updatedCards = $state([...cards]);

  function addCard() {
    updatedCards = [
      ...updatedCards,
      {
        id: "",
        deckId: deck.id,
        front: "",
        back: "",
        mediaUrl: undefined,
      },
    ];
  }

  function removeCard(index: number) {
    updatedCards = updatedCards.filter((_, i) => i !== index);
  }
</script>

<svelte:head>
  <title>{`${m.edit()} ${deck.title} | Flashcards`}</title>
</svelte:head>
<Toolbar>
  <LargeTitle>{deck.title}</LargeTitle>
  <Divider />

  <VStack>
    <Merger>
      <UniButton type="button" onclick={showImportModal} Icon={Import}
        >Import</UniButton
      >
    </Merger>
    <Merger>
      <UniButton
        variant="danger"
        Icon={Trash2}
        formaction="?/delete"
        confirmText={deck.title}
        confirmTitle="Delete Deck"
      >
        {m.delete()}</UniButton
      >

      <UniButton Icon={Ban} href=".">{m.cancel()}</UniButton>
    </Merger>
    <Merger>
      <UniButton variant="prominent" type="submit" Icon={Check}
        >{m.save()}</UniButton
      >
    </Merger>
  </VStack>
</Toolbar>
<form
  method="POST"
  class="flex flex-col gap-4"
  action="?/update"
  use:enhance={enhanceForm({
    messages: {
      redirect: m.changesSaved(),
    },
  })}
>
  <VStack>
    <Input
      labelName="Deck Title"
      name="title"
      placeholder="Give your deck a title"
      value={deck.title}
    />
    <Input
      name="description"
      labelName="Tag 1; Tag2"
      placeholder="What's this deck about?"
      value={deck.description}
    />

    {#if role === "t"}
      <Input name="Visibility" type="visibility" />
      <Input name="Assignee" item={deck} type="assignee" />
    {/if}
  </VStack>

  {#if updatedCards.length === 0}
    <EmptySpace>
      <Title1>
        {m.noFlashcards()}
      </Title1>

      <Merger>
        <UniButton Icon={Plus}>
          {m.new()}
        </UniButton>
      </Merger>
    </EmptySpace>
  {:else}
    <HStack>
      {#each updatedCards as card, index (index)}
        <FlashCardEdit {card} {index} {removeCard} />
      {/each}
    </HStack>
    <UniButton onclick={addCard}>
      {m.new()}
    </UniButton>
  {/if}
  <div class="relative h-full lg:col-span-1"></div>
</form>

{#if page.state.showImportModal}
  <CSV {deck} bind:updatedCards />
{/if}
