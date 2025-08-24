<script lang="ts">
  let { children } = $props();
</script>

<div
  class="ring-default flex gap-3 rounded-full p-1 shadow-md dark:bg-stone-900"
>
  {@render children()}
</div>
