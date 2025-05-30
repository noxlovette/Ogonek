<script>
  import { fly } from "svelte/transition";
  import { cubicOut } from "svelte/easing";

  let { children } = $props();
</script>

<div
  class="ring-default bg-default flex flex-col items-center justify-between gap-2 rounded-lg p-2 shadow-sm md:flex-row md:p-4"
  in:fly={{ x: 5, duration: 400, easing: cubicOut }}
>
  {@render children?.()}
</div>
