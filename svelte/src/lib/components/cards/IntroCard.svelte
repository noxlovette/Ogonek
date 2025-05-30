<script lang="ts">
  import { H1 } from "../typography";

  let { children, title = "Title" } = $props();
</script>

<section
  class="ring-default dark:bg-stone-950 max-w-xs space-y-2 rounded-md bg-white p-5 pb-12 shadow-sm "
>
  <H1>
    {title}
  </H1>
  <p class="text-balance">
    {@render children?.()}
  </p>
</section>
