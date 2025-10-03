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
    EmptySpace,
    DeleteButton,
    CancelButton,
    SaveButton,
    NewCard,
    Caption1,
  } from "$lib/components";

  import { enhanceForm } from "$lib/utils";
  import { enhance } from "$app/forms";
  import { page } from "$app/state";
  import { Import } from "@lucide/svelte";
  import UniButton from "$lib/components/UI/forms/buttons/UniButton.svelte";
  import { pushState } from "$app/navigation";
  import { onMount } from "svelte";
  import { m } from "$lib/paraglide/messages.js";

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
  let { data, form } = $props();
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

  let visibility = $state(deck.visibility);
</script>

<svelte:head>
  <title>{m.edit()} • {deck.title}</title>
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
        <UniButton
          content={m.legal_suave_poodle_edit()}
          type="button"
          onclick={showImportModal}
          Icon={Import}
        ></UniButton>
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
          <DeleteButton />
        </form>

        <CancelButton />
      </Merger>
      <Merger>
        <SaveButton />
      </Merger>
    </VStack>
  </Toolbar>

  <VStack>
    <Input
      labelName="Название колоды"
      name="title"
      invalid={form?.title}
      invalidDescription="Должно быть название"
      placeholder="Чтобы не потерять"
      value={deck.title}
    />
    <Input
      name="description"
      labelName="Теги через ;"
      placeholder="О чем эта дека?"
      value={deck.description}
    />

    <Divider />
    {#if role === "t"}
      {#if visibility === "assigned"}
        <Input
          name="assignee"
          placeholder="Для кого колода"
          labelName="Назначено"
          invalid={form?.assignee}
          invalidDescription="Для кого колода?"
          item={deck}
          type="assignee"
        />
      {/if}
      <Input
        name="visibility"
        labelName="Кто видит"
        bind:value={visibility}
        type="visibility"
      />
    {/if}
  </VStack>

  {#if form?.cards}
    <Caption1 override="text-red-500">
      У всех ли карточек заполнены обе стороны?
    </Caption1>
  {/if}

  {#if updatedCards.length === 0}
    <div
      class="flex min-h-40 w-full flex-col items-center justify-center gap-2 rounded-lg border-2 border-dashed border-stone-300/30 p-4 text-center dark:border-stone-700"
    >
      <NewCard {addCard} />
    </div>
  {:else}
    <div class="grid auto-rows-fr items-stretch gap-4 md:grid-cols-2">
      {#each updatedCards as card, index (index)}
        <FlashCardEdit {card} {index} {removeCard} />
      {/each}
      <NewCard {addCard} />
    </div>
  {/if}
</form>

{#if page.state.showImportModal}
  <CSV {deck} bind:updatedCards />
{/if}
