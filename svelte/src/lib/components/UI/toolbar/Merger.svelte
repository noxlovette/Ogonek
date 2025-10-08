<script lang="ts">
  let { children, styling = "bg-default" } = $props();
</script>

<div
  class="ring-default {styling} flex items-center justify-center gap-1 rounded-full shadow-md"
>
  {@render children()}
</div>
