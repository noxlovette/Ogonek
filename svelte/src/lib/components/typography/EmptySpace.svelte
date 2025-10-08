<script lang="ts">
  import { Callout } from ".";

  let { children } = $props();
</script>

<div
  class="flex min-h-40 w-full flex-col items-center justify-center gap-2 rounded-lg border-2 border-dashed border-stone-300/30 p-3.5 text-center dark:border-stone-700"
>
  <Callout>
    {@render children?.()}
  </Callout>
</div>
