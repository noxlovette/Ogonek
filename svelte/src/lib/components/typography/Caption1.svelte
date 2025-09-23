<script lang="ts">
  let {
    id = "caption",
    children,
    styling = "text-stone-500 dark:text-stone-400",
  } = $props();
</script>

<p {id} class="text-xs font-medium {styling}">
  {@render children?.()}
</p>
