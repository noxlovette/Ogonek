<script>
  import { fly } from "svelte/transition";
  import { cubicOut } from "svelte/easing";

  let { children } = $props();
</script>

<div
  class="flex flex-col items-center justify-between gap-2 rounded-lg bg-stone-50/30 p-2 shadow-sm ring ring-stone-300/40 md:flex-row md:p-4 dark:bg-stone-900/30 dark:ring-stone-600/50"
  in:fly={{ x: 5, duration: 400, easing: cubicOut }}
>
  {@render children?.()}
</div>
