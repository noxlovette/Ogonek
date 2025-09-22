<script lang="ts">
  import BackButton from "./BackButton.svelte";

  let { children, src = "", alt = "", override = false } = $props();
</script>

<div
  class="relative flex w-full flex-col justify-between gap-4 py-2 md:py-3 lg:py-4"
>
  {#if src}
    <div class="absolute inset-0 opacity-50">
      <img class="size-full rounded-xl object-cover" {src} {alt} />
    </div>
  {/if}
  <BackButton {override}></BackButton>

  <div class="relative z-10 flex items-end">
    {@render children?.()}
  </div>
</div>
