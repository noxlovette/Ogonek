<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    children,
    styling = "items-end",
    align = "left",
    override = "gap-default",
  }: {
    children: Snippet;
    styling?: string;
    align?: "center" | "left" | "right";
    override?: string;
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

<div class="relative flex h-full {override} {layout} {styling}">
  {@render children()}
</div>
