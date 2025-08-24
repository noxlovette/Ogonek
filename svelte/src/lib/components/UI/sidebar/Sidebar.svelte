<script lang="ts">
  import type { Snippet } from "svelte";
  let { children }: { children?: Snippet } = $props();
</script>

<div
  class={`
    ring-default bg-default bg-default relative z-30
    flex h-max w-full
    flex-shrink-0 flex-col rounded-2xl
    shadow-lg 
  `}
>
  <div class="w-full overflow-hidden py-2">
    <ul
      class="flex flex-col gap-4 divide-stone-200/40 px-3 dark:divide-stone-600/80"
    >
      {@render children?.()}
    </ul>
  </div>
</div>
