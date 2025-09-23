<script lang="ts">
  import { Callout, Title1 } from "../typography";

  let { children, title = "Title" } = $props();
</script>

<section
  class="flex flex-col items-center gap-2 md:max-w-md md:items-start md:gap-3 lg:gap-4"
>
  <Title1>
    {title}
  </Title1>

  <Callout>
    {@render children?.()}
  </Callout>
</section>
