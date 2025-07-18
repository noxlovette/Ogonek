<script lang="ts">
  import {
    Label,
    Input,
    H1,
    FlashCardEdit,
    AssigneeSelector,
    CSV,
    HeaderEmbellish,
  } from "$lib/components";

  import { enhanceForm } from "$lib/utils";
  import { enhance } from "$app/forms";
  import { page } from "$app/state";
  import { Plus, UploadCloud, Ban, Check, Trash2 } from "lucide-svelte";
  import UniButton from "$lib/components/UI/UniButton.svelte";
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
  <title>{`${m.edit()} ${deck.name} | Flashcards`}</title>
</svelte:head>
<HeaderEmbellish>
  <H1>{deck.name}</H1>
  <div class="hidden md:block">
    <div class="flex items-center gap-3">
      <span class="text-sm text-stone-500 dark:text-stone-400">
        {updatedCards.length}
        {updatedCards.length === 1 ? "card" : "cards"}
      </span>
    </div>
  </div>
</HeaderEmbellish>
<form
  method="POST"
  action="?/update"
  class="space-y-8"
  use:enhance={enhanceForm({
    messages: {
      redirect: m.changesSaved(),
    },
  })}
>
  <input type="hidden" value={deck.id} name="id" />
  <input type="hidden" value={deck.assignee} name="initialAssignee" />

  <div class="grid grid-cols-1 gap-6 lg:grid-cols-4">
    <div class="lg:col-span-3">
      <div class="space-y-6">
        {#if updatedCards.length === 0}
          <div
            class="bg-default flex flex-col items-center justify-center rounded-lg border-2 border-dashed border-stone-300/30 p-12 text-center dark:border-stone-600/30 dark:bg-stone-900/50"
          >
            <div class="mb-4 rounded-full bg-stone-100 p-4 dark:bg-stone-800">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-8 w-8 text-stone-500 dark:text-stone-400"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M9 13h6m-3-3v6m-9 1V7a2 2 0 012-2h6l2 2h6a2 2 0 012 2v8a2 2 0 01-2 2H5a2 2 0 01-2-2z"
                />
              </svg>
            </div>
            <h3 class="text-lg font-medium text-stone-700 dark:text-stone-300">
              {m.noFlashcards()}
            </h3>
            <p class="mt-1 max-w-md text-stone-500 dark:text-stone-400">
              {m.deft_fuzzy_stingray_push()}
            </p>
            <button
              type="button"
              class="bg-cacao-500 hover:bg-cacao-600 focus:ring-cacao-500 dark:bg-cacao-600 dark:hover:bg-cacao-700 mt-6 inline-flex items-center rounded-lg px-4 py-2 text-sm font-medium text-stone-50 shadow-sm transition focus:ring-2 focus:ring-offset-2 focus:outline-none dark:focus:ring-offset-stone-900"
              onclick={addCard}
            >
              <Plus class="mr-2 h-4 w-4" />
              {m.new()}
            </button>
          </div>
        {:else}
          <div class="space-y-4">
            {#each updatedCards as card, index (index)}
              <FlashCardEdit {card} {index} {removeCard} />
            {/each}
          </div>

          <button
            type="button"
            onclick={addCard}
            class="group hover:border-cacao-300 hover:text-cacao-500 dark:hover:border-cacao-700 dark:hover:text-cacao-400 bg-default mt-4 flex w-full items-center justify-center rounded-lg border-2 border-dashed border-stone-300/30 py-4 text-stone-500 transition dark:border-stone-600/30 dark:bg-stone-900/50 dark:text-stone-400"
          >
            <Plus class="mr-2 h-5 w-5 transition group-hover:scale-110" />
            <span>{m.new()}</span>
          </button>
        {/if}
      </div>
    </div>
    <div class="relative h-full lg:col-span-1">
      <div class="sticky top-6 space-y-6">
        <div>
          <Input
            labelName="Deck Name"
            name="name"
            type="text"
            placeholder="Give your deck a name"
            value={deck.name}
          />
        </div>

        <div>
          <Input
            name="description"
            labelName="Tag 1; Tag2"
            placeholder="What's this deck about?"
            type="textarea"
            value={deck.description}
          />
        </div>

        {#if role === "t"}
          <AssigneeSelector item={deck} />

          <div>
            <Label>{m.visibility()}</Label>
            <select
              name="visibility"
              value={deck.visibility}
              class="focus:border-cacao-500 focus:ring-cacao-500/20 h-full w-full rounded-2xl border border-stone-300 bg-white px-4 py-2 text-base text-stone-900 placeholder-stone-400 shadow-sm transition-all focus:shadow-md focus:ring-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-60 dark:border-stone-700 dark:bg-stone-950 dark:text-stone-100"
            >
              <option value="private">{m.private()}</option>
              <option value="public">{m.public()}</option>
              <option value="assigned">{m.assigned()}</option>
            </select>
          </div>
        {/if}

        <div class="flex flex-col gap-3 pt-4">
          <UniButton
            type="button"
            onclick={showImportModal}
            Icon={UploadCloud}
            variant="ghost">Import</UniButton
          >
          <UniButton variant="secondary" Icon={Ban} href="."
            >{m.cancel()}</UniButton
          >
          <UniButton variant="primary" type="submit" Icon={Check}
            >{m.save()}</UniButton
          >
          <UniButton
            variant="danger"
            Icon={Trash2}
            formaction="?/delete"
            confirmText={deck.name}
            confirmTitle="Delete Deck"
          >
            {m.delete()}</UniButton
          >
        </div>
      </div>
    </div>
  </div>
</form>

{#if page.state.showImportModal}
  <CSV {deck} bind:updatedCards />
{/if}
