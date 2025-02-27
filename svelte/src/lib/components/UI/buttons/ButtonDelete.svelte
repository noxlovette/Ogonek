<script lang="ts">
  import ButtonRaw from "./ButtonRaw.svelte";

  let { isSubmitting = $bindable(false), text = "this thing" } = $props();

  let confirmDelete = $state(false);
  const type = "submit";
  const buttonName = "Delete";
  const formaction = "?/delete";
</script>

<ButtonRaw
  {buttonName}
  onclick={() => (confirmDelete = true)}
  formaction={undefined}
  type="button"
></ButtonRaw>
{#if confirmDelete}
  <div
    class="bg-milk-950/50 fixed inset-0 z-50 flex items-center justify-center p-4"
  >
    <div
      class="dark:bg-milk-900 w-full max-w-md rounded-xl bg-white p-6 shadow-xl"
    >
      <h3 class="text-milk-800 dark:text-milk-200 text-xl font-semibold">
        Delete Deck
      </h3>
      <p class="text-milk-600 dark:text-milk-400 mt-2">
        Are you sure you want to delete <span
          class="text-milk-800 dark:text-milk-200 font-medium">"{text}"</span
        >? This action cannot be undone.
      </p>
      <div class="mt-6 flex justify-end gap-3">
        <button
          type="button"
          class="border-milk-200 text-milk-700 hover:bg-milk-50 focus:ring-cacao-500 dark:border-milk-800 dark:bg-milk-900 dark:text-milk-300 dark:hover:bg-milk-800 dark:focus:ring-offset-milk-900 rounded-lg border bg-white px-4 py-2 text-sm font-medium shadow-sm transition focus:ring-2 focus:ring-offset-2 focus:outline-none"
          onclick={() => (confirmDelete = false)}
        >
          Cancel
        </button>
        <ButtonRaw
          {type}
          {buttonName}
          {formaction}
          styling="bg-red-600 hover:bg-red-700"
          bind:isSubmitting
        />
      </div>
    </div>
  </div>
{/if}
