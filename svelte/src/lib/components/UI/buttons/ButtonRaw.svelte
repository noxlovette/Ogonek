<script lang="ts">
  import { Loader2 } from "lucide-svelte";
  let {
    isSubmitting = $bindable(false),
    onclick = undefined,
    formaction = undefined,
    type = undefined,
    styling = "",
    disabled = false,
    buttonName,
  } = $props();
</script>

<button
  {type}
  {formaction}
  {onclick}
  disabled={isSubmitting || disabled}
  class={[
    styling,
    "dark:hover:from-milk-900 dark:hover:to-milk-950 ring-milk-200 dark:ring-milk-800 dark:hover:ring-milk-700 from-cacao-500 to-cacao-600 text-cacao-50 dark:from-milk-800 dark:to-milk-900 dark:text-cacao-100 hover:bg-cacao-700 focus:ring-cacao-500 flex items-center justify-center rounded-lg bg-gradient-to-br px-3 py-2 text-sm ring transition-all focus:ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50 md:px-4 md:text-base",
  ]}
>
  {#if isSubmitting}
    <Loader2 class="mr-1 size-5 animate-spin" />
  {/if}
  {isSubmitting ? "On It..." : buttonName}
</button>
