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
    disabled = false,
    glowColor = "cacao-500",
    glowIntensity = "medium",
  } = $props();

  const glowStyles = {
    low: "dark:hover:bg-opacity-15 dark:hover:shadow-inner-sm",
    medium: "dark:hover:bg-opacity-20 dark:hover:shadow-inner",
    high: "dark:hover:bg-opacity-25 dark:hover:shadow-inner-lg",
  };

  const glowIntensityClass = glowStyles[glowIntensity] || glowStyles.medium;

  let loading = $derived(navigating.to);
</script>

<a
  {href}
  aria-label={ariaLabel}
  data-sveltekit-prefetch={prefetch ? "" : null}
  aria-disabled={disabled}
  class={[
    styling,
    `ring-milk-200 dark:ring-milk-900 shadow-milk-100 dark:shadow-milk-900
     dark:bg-milk-950 relative flex flex-col overflow-hidden rounded-xl bg-white p-5
     ring-1 transition-all duration-200 ease-out dark:shadow-none`,
    !disabled
      ? `hover:ring-cacao-400 hover:shadow-cacao-100 dark:hover:shadow-cacao-100
                  dark:hover:ring-${glowColor} dark:hover:shadow-${glowColor}/20
         dark:hover:bg-${glowColor} ${glowIntensityClass}`
      : "",
    disabled ? "cursor-not-allowed opacity-70" : "",
    loading ? "animate-pulse" : "",
  ]
    .filter(Boolean)
    .join(" ")}
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

  {#if !disabled}
    <div
      class="pointer-events-none absolute inset-0 z-0 rounded-xl opacity-0 transition-opacity duration-300 ease-in-out dark:group-hover:opacity-100"
      style={`background: radial-gradient(circle at center, var(--tw-${glowColor.replace("-", "-")}-rgb, 146, 64, 14, 0.15), transparent 70%);`}
    ></div>
  {/if}
</a>
