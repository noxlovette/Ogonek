<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    href,
    children,
    styling = "",
    ariaLabel = "",
  }: {
    href: string;
    styling?: string;
    ariaLabel?: string;
    children?: Snippet;
  } = $props();
</script>

<a
  {href}
  aria-label={ariaLabel}
  class="group bg-clickable hover-default ring-default relative
     flex min-h-[100px] w-full flex-col
     justify-between
     rounded-xl px-3 py-2.5 shadow-md backdrop-blur-md active:bg-stone-100 md:py-4
{styling}
     "
>
  {@render children?.()}
</a>
