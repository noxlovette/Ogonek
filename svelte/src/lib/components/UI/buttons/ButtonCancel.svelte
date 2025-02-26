<script lang="ts">
  import Anchor from "./Anchor.svelte";

  let { text = "Cancel" } = $props();
</script>

<a
  href="."
  class="text-milk-700 from-milk-100 to-milk-200 hover:from-milk-300 hover:to-milk-400 ring-milk-300 rounded-lg bg-gradient-to-bl px-3 py-2 text-center ring transition-colors"
  >{text}</a
>
