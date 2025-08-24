<!-- Updated Panel.svelte -->
<script lang="ts">
  let { children, variant = "default" } = $props();
</script>

<section
  class="overflow-hidden rounded-xl border border-stone-200 bg-white shadow-sm dark:border-stone-800 dark:bg-stone-900"
  class:bg-stone-50={variant === "inset"}
  class:dark:bg-stone-950={variant === "inset"}
>
  <div class="p-6">
    {@render children?.()}
  </div>
</section>
