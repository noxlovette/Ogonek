<script lang="ts">
  import Anchor from "./Anchor.svelte";
  let { href = "/", text = "Edit" } = $props();
</script>

<Anchor {href}>
  {text}
</Anchor>
