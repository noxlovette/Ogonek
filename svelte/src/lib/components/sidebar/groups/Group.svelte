<script lang="ts">
  import { sidebar } from "$lib/stores";

  let { children } = $props();
</script>

<div
  class="flex flex-col rounded-lg p-2 shadow-md ring ring-stone-200 lg:p-3 xl:p-4 dark:ring-stone-900"
>
  {@render children()}
</div>
