<script lang="ts">
  import {
    MetaData,
    Label,
    ButtonSubmit,
    Input,
    ButtonCancel,
    H1,
    FlashCardEdit,
    ButtonDelete,
  } from "$lib/components";
  import { notification } from "$lib/stores";
  import { enhance } from "$app/forms";
  import { Plus } from "lucide-svelte";

  let isSubmitting = $state(false);

  let { data } = $props();
  let { deck, cards } = data;

  let updatedCards = $state([...cards]);

  function addCard() {
    updatedCards = [
      ...updatedCards,
      {
        id: undefined,
        deck_id: deck.id,
        front: "",
        back: "",
        media_url: undefined,
      },
    ];
  }

  function removeCard(index: number) {
    updatedCards = updatedCards.filter((_, i) => i !== index);
  }
</script>

<MetaData title={`Edit ${deck.name} | Flashcards`} robots="noindex, nofollow" />

<H1>{deck.name}</H1>

<form
  method="POST"
  action="?/update"
  class="space-y-8"
  use:enhance={() => {
    isSubmitting = true;

    return async ({ result, update }) => {
      isSubmitting = false;

      if (result.type === "redirect") {
        notification.set({
          message: "Deck updated successfully",
          type: "success",
        });
        update();
      } else if (result.type === "failure") {
        notification.set({
          message: String(result.data?.message) || "Failed to update deck",
          type: "error",
        });
      } else if (result.type === "error") {
        notification.set({
          message: String(result.error?.message) || "An error occurred",
          type: "error",
        });
      }
    };
  }}
>
  <input type="hidden" value={deck.id} name="id" />

  <!-- Main Container -->
  <div class="flex gap-4">
    <div class="dark:bg-milk-900 flex-1 space-y-6 rounded-xl p-6 shadow-sm">
      <div class="flex items-center justify-between">
        <h2 class="text-xl font-semibold">Flashcards</h2>
        <button
          type="button"
          onclick={addCard}
          class="text-cacao-50 bg-cacao-600 hover:bg-cacao-700 inline-flex items-center rounded-lg px-4 py-2 transition-colors"
        >
          <Plus class="mr-2 h-4 w-4" />
          Add Card
        </button>
      </div>

      <div class="space-y-4">
        {#each updatedCards as card, index}
          <FlashCardEdit {card} {index} {removeCard} />
        {/each}
      </div>
    </div>

    <div class="dark:bg-milk-900 w-1/4 rounded-xl p-4 shadow-sm">
      <Label>Deck Name</Label>
      <Input
        name="name"
        type="text"
        placeholder="Give your deck a name"
        value={deck.name}
      />

      <Label>Description</Label>
      <Input
        name="description"
        placeholder="What's this deck about?"
        type="textarea"
        value={deck.description}
      />

      <Label>Sharing</Label>
      <div class="mt-2">
        <label class="flex items-center space-x-2">
          <input
            type="checkbox"
            name="shared"
            bind:checked={deck.shared}
            class="border-milk-300 text-cacao-600 focus:ring-cacao-500 h-4 w-4 rounded"
          />
          <span class="text-milk-700 dark:text-milk-300 text-sm">
            Share with connected students/teachers
          </span>
        </label>
      </div>
    </div>
  </div>

  <div class="flex justify-end space-x-4">
    <ButtonCancel />
    <ButtonSubmit></ButtonSubmit>
    <ButtonDelete></ButtonDelete>
  </div>
</form>
