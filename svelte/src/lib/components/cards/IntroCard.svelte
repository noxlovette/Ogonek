<script lang="ts">
  import { Callout, Title1, Title2 } from "../typography";

  let { children, title = "Title" } = $props();
</script>

<section
  class="flex max-w-md flex-col items-center gap-2 text-center md:gap-3 md:text-left lg:gap-4"
>
  <Title2>
    {title}
  </Title2>

  <Callout>
    {@render children?.()}
  </Callout>
</section>
