<script lang="ts">
  import ModalBackGround from "./forms/ModalBackGround.svelte";
  let {
    confirmTitle = "You are deleting an item",
    confirmText = "Are you sure you want to do this?",
    showConfirmDialog = $bindable(false),
  } = $props();
</script>

<ModalBackGround>
  <h3 class="text-lg font-semibold text-stone-800 dark:text-stone-200">
    {confirmTitle}
  </h3>
  <p class="mt-2 text-sm text-stone-600 dark:text-stone-400">
    {confirmText}
  </p>
  <div class="mt-5 flex justify-end gap-2">
    <button
      type="button"
      class="rounded-lg bg-white px-4 py-2 text-stone-700/30 ring-1 ring-stone-300 hover:bg-stone-50 dark:bg-stone-800/30 dark:text-stone-200 dark:hover:bg-stone-700"
      onclick={() => (showConfirmDialog = false)}
    >
      Cancel
    </button>
    <button
      type="submit"
      class="rounded-lg bg-gradient-to-br from-red-500 to-red-600 px-4 py-2 text-stone-50 ring-1 ring-red-300 hover:from-red-500 hover:to-red-700 focus:ring focus:ring-red-400 focus:ring-offset-2"
    >
      Confirm
    </button>
  </div>
</ModalBackGround>
