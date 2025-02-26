<script lang="ts">
  import { sidebar } from "$lib/stores";

  let { children } = $props();
</script>

<div
  class="dark:ring-milk-900 ring-milk-200 flex flex-col rounded-lg p-2 shadow-md ring lg:p-3 xl:p-4"
>
  {@render children()}
</div>
