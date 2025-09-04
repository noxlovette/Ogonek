<script lang="ts">
  let { children } = $props();
</script>

<div
  class="ring-default flex gap-1 rounded-full bg-stone-100 shadow-md md:gap-2 lg:gap-3 dark:bg-stone-900"
>
  {@render children()}
</div>
