<script lang="ts">
  let { children, soft = false } = $props();
</script>

<div
  aria-label="Dark Overlay"
  class="fixed inset-0 z-50 flex items-center justify-center {soft
    ? 'bg-none'
    : 'bg-black/30'} backdrop-blur-sm"
>
  <div
    class="ring-default w-full max-w-md rounded-2xl bg-stone-100/50 p-3.5 dark:bg-stone-800/50"
  >
    {@render children?.()}
  </div>
</div>
