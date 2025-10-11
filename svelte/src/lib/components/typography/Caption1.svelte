<script lang="ts">
  let {
    id = "caption",
    children,
    override = "text-stone-500 dark:text-stone-400",
    styling = "",
  } = $props();
</script>

<p {id} class="min-w-max text-xs font-medium {styling} {override}">
  {@render children?.()}
</p>
