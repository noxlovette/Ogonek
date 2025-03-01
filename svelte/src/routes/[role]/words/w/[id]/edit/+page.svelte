<script lang="ts">
  import {
    MetaData,
    Label,
    Input,
    H1,
    FlashCardEdit,
    AssigneeSelector,
    CSV,
  } from "$lib/components";
  import { notification } from "$lib/stores";
  import { enhance } from "$app/forms";
  import { page } from "$app/state";
  import { Plus, Upload, UploadCloud, Ban, Check, Trash2 } from "lucide-svelte";
  import UniButton from "$lib/components/UI/UniButton.svelte";

  let showImportModal = $state(false);

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

<MetaData title={`Edit ${deck.name} | Flashcards`} robots="noindex, nofollow" />

<div class="flex items-center justify-between">
  <H1>{deck.name}</H1>
  <div class="hidden md:block">
    <div class="flex items-center gap-3">
      <span class="text-milk-500 dark:text-milk-400 text-sm">
        {updatedCards.length}
        {updatedCards.length === 1 ? "card" : "cards"}
      </span>
    </div>
  </div>
</div>

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

  <div class="grid grid-cols-1 gap-6 lg:grid-cols-4">
    <div class="lg:col-span-3">
      <div class="space-y-6">
        {#if updatedCards.length === 0}
          <div
            class="border-milk-200 bg-milk-50 dark:border-milk-800 dark:bg-milk-900/50 flex flex-col items-center justify-center rounded-xl border-2 border-dashed p-12 text-center"
          >
            <div class="bg-milk-100 dark:bg-milk-800 mb-4 rounded-full p-4">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="text-milk-500 dark:text-milk-400 h-8 w-8"
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
            <h3 class="text-milk-700 dark:text-milk-300 text-lg font-medium">
              No cards yet
            </h3>
            <p class="text-milk-500 dark:text-milk-400 mt-1 max-w-md">
              Create your first flashcard to get started
            </p>
            <button
              type="button"
              class="bg-cacao-500 hover:bg-cacao-600 focus:ring-cacao-500 dark:bg-cacao-600 dark:hover:bg-cacao-700 dark:focus:ring-offset-milk-900 mt-6 inline-flex items-center rounded-lg px-4 py-2 text-sm font-medium text-white shadow-sm transition focus:ring-2 focus:ring-offset-2 focus:outline-none"
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
            class="group border-milk-200 bg-milk-50 text-milk-500 hover:border-cacao-300 hover:text-cacao-500 dark:border-milk-800 dark:bg-milk-900/50 dark:text-milk-400 dark:hover:border-cacao-700 dark:hover:text-cacao-400 mt-4 flex w-full items-center justify-center rounded-lg border-2 border-dashed py-4 transition"
          >
            <Plus class="mr-2 h-5 w-5 transition group-hover:scale-110" />
            <span>Add Another Card</span>
          </button>
        {/if}
      </div>
    </div>
    <div class="lg:col-span-1">
      <div
        class="dark:bg-milk-900 sticky top-6 space-y-6 rounded-xl bg-white p-5 shadow-sm"
      >
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
            onclick={() => (showImportModal = true)}
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
              class="border-milk-200 focus:ring-cacao-500 disabled:text-milk-500 dark:border-milk-800 dark:bg-milk-950 dark:focus:border-milk-800 dark:focus:ring-milk-700 w-full rounded-lg border bg-white px-4 py-2 transition duration-200 focus:ring focus:outline-none dark:focus:ring dark:focus:outline-none"
            >
              <option value="private">Private</option>
              <option value="public">Public</option>
              <option value="assigned">Assigned</option>
            </select>
          </div>
        {/if}

        <div class="flex flex-col gap-3 pt-4">
          <UniButton variant="secondary" Icon={Ban} href=".">Cancel</UniButton>
          <UniButton variant="primary" Icon={Check}>Save</UniButton>
          <UniButton variant="danger" Icon={Trash2} formaction="?/delete">
            Delete</UniButton
          >
        </div>
      </div>
    </div>
  </div>
</form>
{#if showImportModal}
  <CSV bind:updatedCards bind:showImportModal {deck} />
{/if}
