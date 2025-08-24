<script lang="ts">
  import { LargeTitle } from "../typography";

  let { children, title = "Title" } = $props();
</script>

<section
  class="ring-default max-w-xs space-y-2 rounded-md bg-white p-5 pb-12 shadow-sm dark:bg-stone-950"
>
  <LargeTitle>
    {title}
  </LargeTitle>
  <p class="text-balance">
    {@render children?.()}
  </p>
</section>
