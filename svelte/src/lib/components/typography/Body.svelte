<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<p
  class="text-base leading-relaxed text-stone-800 dark:text-stone-400 {styling}"
>
  {@render children?.()}
</p>
