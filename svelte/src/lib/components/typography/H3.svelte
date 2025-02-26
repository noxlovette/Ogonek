<script lang="ts">
  let { children } = $props();
</script>

<h2
  class="text-md snug text-milk-500 font-medium text-balance sm:text-lg lg:text-xl"
>
  {@render children()}
</h2>
