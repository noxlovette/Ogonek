<script lang="ts">
  const { children } = $props();
</script>

<div
  class="bg-solid items-end rounded-b-2xl border-t-2 border-stone-100 dark:border-stone-800"
>
  {@render children()}
</div>
