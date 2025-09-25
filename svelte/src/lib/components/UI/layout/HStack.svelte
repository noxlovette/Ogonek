<script lang="ts">
  let { children, styling = "", override = "gap-default" } = $props();
</script>

<div class="flex w-full flex-col {override} {styling}">
  {@render children()}
</div>
