<script lang="ts">
  let { children } = $props();
</script>

<div
  class="ring-default bg-default flex h-max flex-col space-y-3 rounded-l-lg p-2 shadow-sm transition-all duration-300 hover:shadow-lg lg:p-3 xl:p-4"
>
  {@render children()}
</div>
