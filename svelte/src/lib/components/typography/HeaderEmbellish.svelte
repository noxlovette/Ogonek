<script>
  import { fly } from "svelte/transition";
  import { cubicOut } from "svelte/easing";

  let { children } = $props();
</script>

<div
  class="relative mb-4 flex flex-col items-center gap-3"
  in:fly={{ x: 5, duration: 400, easing: cubicOut }}
>
  {@render children?.()}
</div>
