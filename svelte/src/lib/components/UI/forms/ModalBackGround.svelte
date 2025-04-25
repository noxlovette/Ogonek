<script lang="ts">
  let { children } = $props();
</script>

<div
  class="fixed inset-0 z-50 flex items-center justify-center bg-black/30 backdrop-blur-sm"
>
  <div
    class="w-full max-w-sm rounded-xl bg-white/90 p-6 shadow-sm ring-1 ring-stone-300/40 dark:bg-stone-900/90 dark:ring-stone-600/50"
  >
    {@render children?.()}
  </div>
</div>
