<script lang="ts">
  import type { Snippet } from "svelte";
  interface Props {
    children: Snippet;
  }

  let { children }: Props = $props();
</script>

<div class="ring-default bg-solid rounded-lg shadow-lg" aria-label="Data table">
  {@render children()}
</div>
