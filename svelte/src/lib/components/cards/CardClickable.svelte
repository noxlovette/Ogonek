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
    `ring-milk-200 dark:ring-milk-900 shadow-milk-100 dark:shadow-milk-900
     dark:bg-milk-950 hover:ring-cacao-400 hover:shadow-cacao-100 dark:hover:shadow-cacao-100 dark:hover:bg-opacity-20 relative flex flex-col
     overflow-hidden rounded-xl bg-white p-5 ring-1
      transition-all duration-200 ease-out
    dark:shadow-none
  `,
  ]}
  style={`height: ${height};`}
>
  {#if loading}
    <div
      class="dark:bg-milk-950/70 absolute inset-0 z-10 flex items-center justify-center bg-white/70"
    >
      <div
        class="border-milk-200 border-t-cacao-500 dark:border-milk-700 dark:border-t-cacao-400 h-8 w-8 animate-spin rounded-full border-4"
      ></div>
    </div>
  {/if}

  <div class="relative z-0 h-full">
    {@render children()}
  </div>

  {#if gradient}
    <div
      class="dark:from-milk-950 pointer-events-none absolute right-0 bottom-0 left-0
       z-0 h-12 bg-gradient-to-t from-white
       to-transparent"
    ></div>
  {/if}
</a>
