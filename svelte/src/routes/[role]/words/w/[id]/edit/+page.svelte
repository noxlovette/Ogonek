<script lang="ts">
  import {
    MetaData,
    Label,
    ButtonSubmit,
    Input,
    ButtonCancel,
    H1,
  } from "$lib/components";
  import { notification } from "$lib/stores";
  import { enhance } from "$app/forms";
  import { Trash2, Plus } from "lucide-svelte";

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
    <div
      class="bg-milk-50 dark:bg-milk-900 flex-1 space-y-6 rounded-xl p-6 shadow-sm"
    >
      <div class="flex items-center justify-between">
        <h2 class="text-xl font-semibold">Flashcards</h2>
        <button
          type="button"
          onclick={addCard}
          class="bg-cocoa-500 hover:bg-cocoa-600 inline-flex items-center rounded-lg px-4 py-2 text-white transition-colors"
        >
          <Plus class="mr-2 h-4 w-4" />
          Add Card
        </button>
      </div>

      <div class="space-y-4">
        {#each updatedCards as card, index}
          <div class="dark:bg-milk-800 rounded-lg bg-white p-4 shadow-sm">
            <div class="mb-4 flex justify-end">
              <button
                type="button"
                onclick={() => removeCard(index)}
                class="rounded-full bg-red-500 p-2 text-white transition hover:bg-red-600 focus:ring-2 focus:ring-red-400"
                title="Remove card"
              >
                <Trash2 class="h-5 w-5" />
              </button>
            </div>

            <div class="grid gap-4 md:grid-cols-2">
              <!-- Front of card -->
              <div>
                <Label>Front</Label>
                <Input
                  name={`cards[${index}][front]`}
                  type="textarea"
                  bind:value={card.front}
                  placeholder="Question or prompt"
                />
              </div>

              <!-- Back of card -->
              <div>
                <Label>Back</Label>
                <Input
                  name={`cards[${index}][back]`}
                  type="textarea"
                  bind:value={card.back}
                  placeholder="Answer or explanation"
                />
              </div>

              <!-- Media URL (optional) -->
              <div class="md:col-span-2">
                <Label>Media URL (optional)</Label>
                <Input
                  name={`cards[${index}][media_url]`}
                  bind:value={card.media_url}
                  placeholder="https://example.com/image.jpg"
                />
              </div>
            </div>
          </div>
        {/each}
      </div>
    </div>

    <div class="bg-milk-50 dark:bg-milk-900 w-1/4 rounded-xl p-4 shadow-sm">
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
            class="border-milk-300 text-cocoa-600 focus:ring-cocoa-500 h-4 w-4 rounded"
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
  </div>
</form>
