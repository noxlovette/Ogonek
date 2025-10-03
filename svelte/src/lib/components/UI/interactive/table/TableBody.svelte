<script lang="ts">
  import { fade } from "svelte/transition";

  const { children } = $props();
</script>

<div class="divide-y divide-stone-200 dark:divide-stone-800" in:fade|global>
  {@render children()}
</div>
