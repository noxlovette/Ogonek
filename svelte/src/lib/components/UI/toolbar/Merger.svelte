<script lang="ts">
  let { children, styling = "bg-default" } = $props();
</script>

<div
  class="ring-default {styling} flex gap-1 rounded-full shadow-md md:gap-2 lg:gap-3"
>
  {@render children()}
</div>
