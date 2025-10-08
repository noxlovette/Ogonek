<script lang="ts">
  import { Header, WorkArea } from "$lib/components";
  import type { LayoutProps } from "./$types";

  let { data, children }: LayoutProps = $props();
</script>

<Header></Header>
<WorkArea>
  {@render children()}
</WorkArea>
