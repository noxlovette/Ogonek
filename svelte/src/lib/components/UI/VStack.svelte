<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    children,
    styling = "",
    align = "left",
  }: {
    children: Snippet;
    styling?: string;
    align?: "center" | "left" | "right";
  } = $props();

  const layout = $derived.by(() => {
    switch (align) {
      case "center":
        return "justify-center";
      case "left":
        return "justify-start";
      case "right":
        return "justify-end";
    }
  });
</script>

<div class="relative flex h-full items-end gap-4 {layout} {styling}">
  {@render children()}
</div>
