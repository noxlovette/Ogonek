<script lang="ts">
  import ButtonRaw from "./ButtonRaw.svelte";

  let {
    isSubmitting = $bindable(false),
    text = "this thing",
    title = "this thing",
  } = $props();

  let confirmDelete = $state(false);
  const type = "submit";
  const buttonName = "Delete";
  const formaction = "?/delete";
</script>

<ButtonRaw
  {buttonName}
  onclick={() => (confirmDelete = true)}
  formaction={undefined}
  styling="from-red-500 to-red-600 dark:from-red-500 dark:to-red-600 hover:from-red-500 hover:to-red-700 dark:hover:from-red-500 dark:hover:to-red-700"
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
        Delete {title}
      </h3>
      <p class="text-milk-600 dark:text-milk-400 mt-2">
        Are you sure you want to delete <span
          class="text-milk-800 dark:text-milk-200 font-medium">{text}</span
        >? This action cannot be undone.
      </p>
      <div class="mt-6 flex justify-end gap-3">
        <button
          type="button"
          class="text-milk-700 from-milk-50 to-milk-100 hover:to-milk-200 ring-milk-300 rounded-lg bg-gradient-to-bl px-3 py-2 text-center ring transition-colors"
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
