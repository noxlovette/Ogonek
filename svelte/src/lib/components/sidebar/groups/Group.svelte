<script lang="ts">
  let { children } = $props();
</script>

<div
  class="ring-default lg:p-3 flex h-max flex-col space-y-3 rounded-l-lg p-2 shadow-sm xl:p-4 bg-default "
>
  {@render children()}
</div>
