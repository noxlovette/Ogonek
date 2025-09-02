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
  import UniButton from "$lib/components/UI/forms/UniButton.svelte";
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
        <form
          method="POST"
          use:enhance={enhanceForm({
            messages: {
              redirect: m.tiny_happy_rat_bless(),
            },
          })}
        >
          <UniButton
            variant="danger"
            Icon={Trash2}
            type="submit"
            formaction="?/delete"
            confirmText={deck.title}
            confirmTitle="Delete Deck"
          >
            {m.delete()}</UniButton
          >
        </form>

        <UniButton Icon={Ban} href=".">{m.cancel()}</UniButton>
      </Merger>
      <Merger>
        <UniButton variant="prominent" type="submit" Icon={Check}
          >{m.save()}</UniButton
        >
      </Merger>
    </VStack>
  </Toolbar>

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
      <Input name="visibility" value={deck.visibility} type="visibility" />
      <Input name="assignee" item={deck} type="assignee" />
    {/if}
  </VStack>

  {#if updatedCards.length === 0}
    <EmptySpace>
      <Title1>
        {m.noFlashcards()}
      </Title1>

      <Merger>
        <UniButton Icon={Plus} onclick={addCard}>
          {m.new()}
        </UniButton>
      </Merger>
    </EmptySpace>
  {:else}
    <div class="grid auto-rows-fr items-stretch gap-4 md:grid-cols-2">
      {#each updatedCards as card, index (index)}
        <FlashCardEdit {card} {index} {removeCard} />
      {/each}
      <button
        type="button"
        class="group focus:ring-accent flex h-full w-full items-center justify-center rounded-2xl border border-dashed border-stone-200 shadow-sm transition focus:ring-2 focus:ring-offset-2 focus:outline-none dark:border-stone-800"
        onclick={addCard}
      >
        <Plus class="group-hover:text-accent" />
      </button>
    </div>
  {/if}
</form>

{#if page.state.showImportModal}
  <CSV {deck} bind:updatedCards />
{/if}
