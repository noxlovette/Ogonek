<script lang="ts">
  import { Callout, LargeTitle, Title1 } from "../typography";

  let { children, title = "Title" } = $props();
</script>

<section class="max-w-md gap-2 md:gap-3 lg:gap-4">
  <Title1>
    {title}
  </Title1>

  <Callout>
    {@render children?.()}
  </Callout>
</section>
