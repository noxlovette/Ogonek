<script lang="ts">
  let { children, styling } = $props();
</script>

<h4
  class="text-md snug overflow-clip tracking-normal text-balance lg:text-xl {styling}"
>
  {@render children()}
</h4>
