<script lang="ts">
  import { navigating } from "$app/state";

  let {
    href,
    children,
    height = "150px",
    styling = "",
    gradient = true,
    prefetch = true,
    ariaLabel = "",
  } = $props();

  let loading = $derived(navigating.to);
</script>

<a
  {href}
  aria-label={ariaLabel}
  data-sveltekit-prefetch={prefetch ? "" : null}
  class={[
    styling,
    `hover:ring-cacao-400 relative
     flex flex-col overflow-hidden rounded-lg bg-white p-5 shadow-sm ring-1
      ring-stone-200 transition-all duration-200 ease-out
      dark:bg-stone-950
    dark:ring-stone-800
  `,
  ]}
  style={`height: ${height};`}
>
  {#if loading}
    <div
      class="absolute inset-0 z-10 flex items-center justify-center bg-white/70 dark:bg-stone-950/70"
    >
      <div
        class="border-t-cacao-500 dark:border-t-cacao-400 h-8 w-8 animate-spin rounded-full border-4 border-stone-200 dark:border-stone-700"
      ></div>
    </div>
  {/if}

  <div class="relative z-0 h-full">
    {@render children()}
  </div>

  {#if gradient}
    <div
      class="pointer-events-none absolute right-0 bottom-0 left-0 z-0
       h-12 bg-gradient-to-t from-white to-transparent
       dark:from-stone-950"
    ></div>
  {/if}
</a>
