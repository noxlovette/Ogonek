<script lang="ts">
  let { children } = $props();
</script>

<div
  class="ring-default bg-default flex h-max flex-col gap-2 rounded-l-lg p-2 shadow-sm hover:shadow-lg lg:p-3 xl:p-4"
>
  {@render children()}
</div>
