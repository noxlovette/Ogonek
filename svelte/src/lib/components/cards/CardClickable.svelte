<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    href,
    children,
    height = "150px",
    styling = "",
    prefetch = true,
    ariaLabel = "",
  }: {
    href: string;
    height?: string;
    styling?: string;
    prefetch?: boolean;
    ariaLabel?: string;
    children?: Snippet;
  } = $props();
</script>

<a
  {href}
  aria-label={ariaLabel}
  data-sveltekit-prefetch={prefetch ? "" : null}
  class={[
    styling,
    `group ring-default bg-clickable relative flex
     flex-col justify-between 
     rounded-xl px-4 py-5 shadow-sm backdrop-blur-md active:bg-stone-100
    `,
  ].join(" ")}
  style={`height: ${height};`}
>
  {@render children?.()}
</a>
