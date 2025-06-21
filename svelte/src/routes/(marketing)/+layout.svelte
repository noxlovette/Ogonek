<script>
  import { Header } from "$lib/components";
  let { children } = $props();
</script>

<Header></Header>
{@render children?.()}
