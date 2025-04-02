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
  <title>{`Edit ${deck.name} | Flashcards`}</title>
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
      redirect: "Deck updated successfully",
      failure: "Failed to update deck",
      error: "An error occurred",
    },
  })}
>
  <input type="hidden" value={deck.id} name="id" />

  <div class="grid grid-cols-1 gap-6 lg:grid-cols-4">
    <div class="lg:col-span-3">
      <div class="space-y-6">
        {#if updatedCards.length === 0}
          <div
            class="flex flex-col items-center justify-center rounded-lg border-2 border-dashed border-stone-200 bg-stone-50 p-12 text-center dark:border-stone-800 dark:bg-stone-900/50"
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
              No cards yet
            </h3>
            <p class="mt-1 max-w-md text-stone-500 dark:text-stone-400">
              Create your first flashcard to get started
            </p>
            <button
              type="button"
              class="bg-cacao-500 hover:bg-cacao-600 focus:ring-cacao-500 dark:bg-cacao-600 dark:hover:bg-cacao-700 mt-6 inline-flex items-center rounded-lg px-4 py-2 text-sm font-medium text-white shadow-sm transition focus:ring-2 focus:ring-offset-2 focus:outline-none dark:focus:ring-offset-stone-900"
              onclick={addCard}
            >
              <Plus class="mr-2 h-4 w-4" />
              Add First Card
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
            class="group hover:border-cacao-300 hover:text-cacao-500 dark:hover:border-cacao-700 dark:hover:text-cacao-400 mt-4 flex w-full items-center justify-center rounded-lg border-2 border-dashed border-stone-200 bg-stone-50 py-4 text-stone-500 transition dark:border-stone-800 dark:bg-stone-900/50 dark:text-stone-400"
          >
            <Plus class="mr-2 h-5 w-5 transition group-hover:scale-110" />
            <span>Add Another Card</span>
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
            placeholder="What's this deck about?"
            type="textarea"
            value={deck.description}
          />
        </div>

        <div>
          <UniButton
            type="button"
            onclick={showImportModal}
            Icon={UploadCloud}
            variant="outline">Import</UniButton
          >
        </div>

        {#if role === "t"}
          <AssigneeSelector item={deck} />

          <div>
            <Label>Visibility</Label>
            <select
              name="visibility"
              value={deck.visibility}
              class="focus:ring-cacao-500 w-full rounded-lg border border-stone-200 bg-white px-4 py-2 transition duration-200 focus:ring focus:outline-none disabled:text-stone-500 dark:border-stone-800 dark:bg-stone-950 dark:focus:border-stone-800 dark:focus:ring dark:focus:ring-stone-700 dark:focus:outline-none"
            >
              <option value="private">Private</option>
              <option value="public">Public</option>
              <option value="assigned">Assigned</option>
            </select>
          </div>
        {/if}

        <div class="flex flex-col gap-3 pt-4">
          <UniButton variant="secondary" Icon={Ban} href=".">Cancel</UniButton>
          <UniButton variant="primary" type="submit" Icon={Check}
            >Save</UniButton
          >
          <UniButton
            variant="danger"
            Icon={Trash2}
            formaction="?/delete"
            confirmText={deck.name}
            confirmTitle="Delete Deck"
          >
            Delete</UniButton
          >
        </div>
      </div>
    </div>
  </div>
</form>
{#if page.state.showImportModal}
  <CSV bind:updatedCards {deck} />
{/if}
