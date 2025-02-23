<script lang="ts">
  let { children } = $props();
</script>

<div
  class="ring-milk-200 dark:ring-milk-900 dark:bg-milk-950 relative mx-4 flex flex-1 scroll-my-6 flex-col gap-4 overflow-x-auto rounded-lg bg-white px-5 py-3 pb-24 shadow-md ring-2 md:mr-0 md:h-full md:px-6 md:pb-3 lg:px-8 lg:py-4"
>
  {@render children()}
</div>
