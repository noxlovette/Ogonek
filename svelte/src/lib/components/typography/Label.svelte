<script lang="ts">
  let { target = "", children } = $props();
</script>

<label
  class="text-sm font-medium text-stone-700 capitalize opacity-60 dark:text-stone-200"
  for={target}>{@render children()}</label
>
