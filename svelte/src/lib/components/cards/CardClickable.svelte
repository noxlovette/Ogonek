<script lang="ts">
  import H3 from "../typography/H3.svelte";
  import H4 from "../typography/H4.svelte";

  let {
    href,
    title = "Title",
    subtitle = "Subtitle",
    caption = "Caption",
    height = "150px",
    styling = "",
    prefetch = true,
    ariaLabel = "",
  } = $props();
</script>

<a
  {href}
  aria-label={ariaLabel}
  data-sveltekit-prefetch={prefetch ? "" : null}
  class={[
    styling,
    `hover:ring-cacao-400 group relative flex
     flex-col justify-between overflow-hidden rounded-lg bg-white px-4 py-5 shadow-sm ring-1
      ring-stone-300/40 transition-all duration-200 ease-out

      dark:bg-stone-950
    dark:ring-stone-600/50
  `,
  ]}
  style={`height: ${height};`}
>
  <H3 styling="w-1/2">
    {title}
  </H3>

  <H4>
    {subtitle}
  </H4>
  <div
    class="pointer-events-none absolute inset-y-0 right-0 z-0 h-full w-1/2 self-end overflow-hidden opacity-0 transition-opacity group-hover:opacity-20"
  >
    <div class="h-full w-full">
      <p class="markdown-mini h-full w-full px-4">
        {@html caption}
      </p>
    </div>
  </div>
</a>
