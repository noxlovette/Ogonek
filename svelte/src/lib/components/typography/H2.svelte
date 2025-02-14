<script lang="ts">
  let { children } = $props();
</script>

<h2
  class="text-lg/snug font-medium text-balance sm:text-xl/snug lg:text-2xl/snug"
>
  {@render children()}
</h2>
