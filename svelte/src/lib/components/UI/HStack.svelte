<script lang="ts">
  let { children, styling = "gap-2 md:gap-3 lg:gap-4", bg = "" } = $props();
</script>

<div class="flex w-full flex-col {styling}">
  {@render children()}
</div>
