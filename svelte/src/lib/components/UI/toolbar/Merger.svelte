<script lang="ts">
  let { children } = $props();
</script>

<div
  class="ring-default bg-default flex gap-1 rounded-full shadow-md md:gap-2 lg:gap-3"
>
  {@render children()}
</div>
