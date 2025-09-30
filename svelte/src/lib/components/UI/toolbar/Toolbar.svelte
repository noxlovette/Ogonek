<script lang="ts">
  import BackButton from "./BackButton.svelte";

  let { children, override = false } = $props();
</script>

<div
  class="relative flex w-full flex-col justify-between gap-4 py-2 md:py-3 lg:py-4"
>
  <BackButton {override}></BackButton>

  <div class="relative z-10 flex items-end">
    {@render children?.()}
  </div>
</div>
