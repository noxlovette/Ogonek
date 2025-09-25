<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<div
  class="gap-default grid items-center md:grid-cols-2 md:items-start {styling}"
>
  {@render children()}
</div>
