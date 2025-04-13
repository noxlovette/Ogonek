<script lang="ts">
  let { children } = $props();
</script>

<h2
  class="text-md snug overflow-clip font-medium text-balance text-stone-500 sm:text-lg lg:text-xl"
>
  {@render children()}
</h2>
