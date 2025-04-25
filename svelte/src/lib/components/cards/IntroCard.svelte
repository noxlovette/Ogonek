<script lang="ts">
  import { H1 } from "../typography";

  let { children, title = "Title" } = $props();
</script>

<section
  class="max-w-xs space-y-2 rounded-md bg-white p-5 pb-12 shadow-sm ring ring-stone-300/40 dark:bg-stone-950 dark:ring-stone-600/50"
>
  <H1>
    {title}
  </H1>
  <p class="text-balance">
    {@render children?.()}
  </p>
</section>
