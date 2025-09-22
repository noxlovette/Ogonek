<script lang="ts">
  let {
    children,
    styling = "",
    override = "gap-2 md:gap-3 lg:gap-4",
  } = $props();
</script>

<div class="flex w-full flex-col {override} {styling}">
  {@render children()}
</div>
