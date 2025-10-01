<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    href,
    children,
    styling = "",
    ariaLabel = "",
    dataCy = "clickable-card",
    deactivate = false,
  }: {
    href: string;
    styling?: string;
    ariaLabel?: string;
    children?: Snippet;
    dataCy?: string;
    deactivate?: boolean;
  } = $props();

  const baseStyle = `group  ring-default relative flex min-h-[100px] w-full flex-col justify-between rounded-xl px-3 py-2.5 shadow-md backdrop-blur-md  md:py-4 ${deactivate ? "bg-default" : "active:bg-stone-100 bg-clickable hover-default"}`;
</script>

{#if !deactivate}
  <a
    {href}
    data-cy={dataCy}
    aria-label={ariaLabel}
    class="
{styling} {baseStyle}
     "
  >
    {@render children?.()}
  </a>
{:else}
  <div
    data-cy={dataCy}
    aria-label={ariaLabel}
    class="
{styling} {baseStyle}
     "
  >
    {@render children?.()}
  </div>
{/if}
