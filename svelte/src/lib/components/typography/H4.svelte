<script lang="ts">
  let { children } = $props();
</script>

<h4 class="text-md snug overflow-clip tracking-normal text-balance lg:text-xl">
  {@render children()}
</h4>
