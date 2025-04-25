<script lang="ts">
  let { children } = $props();
</script>

<section
  class="flex flex-col gap-3 rounded-lg bg-inherit px-5 py-4 shadow-sm ring ring-stone-300/40 transition-all dark:bg-stone-950 dark:ring-stone-600/50"
>
  {@render children?.()}
</section>
