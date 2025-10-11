<script lang="ts">
  import type { Snippet } from "svelte";
  let { children }: { children?: Snippet } = $props();
</script>

<div
  class="
    ring-default bg-default relative z-30
    flex h-max w-full
    flex-shrink-0 flex-col rounded-2xl
    shadow-md"
>
  <div class="w-full overflow-hidden py-3">
    <ul class="flex flex-col gap-2 px-2.5">
      {@render children?.()}
    </ul>
  </div>
</div>
