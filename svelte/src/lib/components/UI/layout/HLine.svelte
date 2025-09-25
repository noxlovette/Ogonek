<script lang="ts">
  const { opacity = 0.1, thickness = 2 } = $props();
</script>

<!-- Séparateur style Apple avec Tailwind -->
<div
  class="w-full bg-stone-400"
  style="height: {thickness}px; opacity: {opacity};"
  role="separator"
  aria-hidden="true"
></div>

<!-- Version alternative avec plus de finesse visuelle -->
<!-- 
<hr class="w-full border-0 h-px bg-gradient-to-r from-transparent via-stone-400 to-transparent dark:via-stone-600 opacity-10" />
-->
