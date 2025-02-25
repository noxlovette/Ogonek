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
    AssigneeSelector,
    ButtonRaw,
  } from "$lib/components";
  import { notification } from "$lib/stores";
  import { enhance } from "$app/forms";
  import { page } from "$app/state";

  const role = page.params.role;

  let isSubmitting = $state(false);

  let { data } = $props();
  let { deck, cards } = data;

  let updatedCards = $state([...cards]);

  function addCard() {
    updatedCards = [
      ...updatedCards,
      {
        id: "",
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

  <div class="flex gap-4">
    <div class="dark:bg-milk-900 flex-1 space-y-6 rounded-xl p-6 shadow-sm">
      <div class="flex items-center justify-between">
        <h2 class="text-xl font-semibold">Flashcards</h2>
      </div>

      <div class="space-y-4">
        {#each updatedCards as card, index}
          <FlashCardEdit {card} {index} {removeCard} />
        {/each}
      </div>
      <ButtonRaw onclick={addCard} type="button" buttonName="Add Card +" />
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

      {#if role === "t"}
        <AssigneeSelector item={deck} />
      {/if}
      <Label>Visibility</Label>
      <select
        name="role"
        required
        class="dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 disabled:text-milk-500 border-milk-200 dark:bg-milk-950 focus:ring-cacao-500 w-full rounded-lg border px-4 py-2
            transition duration-200 focus:ring focus:outline-none
                   dark:focus:ring dark:focus:outline-none"
      >
        <option value="">Public</option>
        <option value="teacher">Private</option>
        {#if role === "t"}
          <option value="student">Shared</option>
        {/if}
      </select>
    </div>
  </div>

  <div class="flex justify-end space-x-4">
    <ButtonCancel />
    <ButtonSubmit></ButtonSubmit>
    <ButtonDelete></ButtonDelete>
  </div>
</form>
