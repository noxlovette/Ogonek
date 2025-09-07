<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<div
  class="grid items-center gap-2 md:grid-cols-2 md:items-start md:gap-3 lg:gap-4 {styling}"
>
  {@render children()}
</div>
