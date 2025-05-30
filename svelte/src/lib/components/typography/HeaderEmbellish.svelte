<script>
  import { fly } from "svelte/transition";
  import { cubicOut } from "svelte/easing";

  let { children } = $props();
</script>

<div
  class="ring-default md:flex-row flex flex-col items-center justify-between gap-2 rounded-lg bg-default p-2 shadow-sm md:p-4 "
  in:fly={{ x: 5, duration: 400, easing: cubicOut }}
>
  {@render children?.()}
</div>
