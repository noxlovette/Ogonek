<script lang="ts">
  import { sidebar } from "$lib/stores/sidebar";
  import { page } from "$app/state";
  import H3 from "$lib/components/typography/H3.svelte";
  import type { Component } from "svelte";
  import { SquareArrowUpRight } from "lucide-svelte";

  let {
    Icon,
    href = "/",
    name,
    external = false,
    rightBar = false,
    badge = undefined,
    disabled = false,
  }: {
    Icon?: Component;
    href: string;
    name: string;
    external?: boolean;
    rightBar?: boolean;
    badge?: string | number;
    disabled?: boolean;
  } = $props();

  const target = $derived(external ? "_blank" : undefined);
  const rel = $derived(external ? "noopener noreferrer" : undefined);
  const isActive = $derived(
    page.url.pathname === href ||
      (href !== "/" && page.url.pathname.startsWith(href)),
  );

  // Clean, modern styling without gradients
  const baseClasses =
    "group relative flex items-center gap-3 active:scale-[0.95] hover:scale-[1.02] px-3 py-3 text-sm font-medium transition-all duration-300 ease-out select-none";

  const stateClasses = $derived(() => {
    if (disabled) return "opacity-40 cursor-not-allowed";
    return "text-stone-700 dark:text-stone-300";
  });

  const iconClasses = $derived(() => {
    const base = "transition-all duration-300 flex-shrink-0";
    if ($sidebar) return `${base} size-6`;
    return `${base} size-5 ${isActive ? "text-cacao-600 dark:text-cacao-400" : "text-stone-500 dark:text-stone-400"}`;
  });

  const badgeVisible = $derived(badge && badge != 0 ? true : false);
</script>

<!-- Collapsed sidebar with clean modern styling -->
{#if $sidebar && !rightBar}
  <div class="group/tooltip relative">
    <a
      {href}
      {target}
      {rel}
      aria-label={name}
      class={`${baseClasses} ${stateClasses} hover:border-cacao-300/50 dark:hover:border-cacao-600/50 min-h-[52px] min-w-[52px]
        justify-center
        rounded-xl border border-stone-200/30
        bg-white/90 shadow-md
        shadow-stone-900/10 backdrop-blur-sm
        hover:shadow-xl hover:shadow-stone-900/15
        active:shadow-inner active:shadow-stone-900/20
        dark:border-stone-700/30 dark:bg-stone-800/90
        ${isActive ? "border-cacao-400/60 dark:border-cacao-500/60 bg-cacao-50/95 dark:bg-cacao-900/80 shadow-cacao-500/20" : ""}
        transform hover:-translate-y-0.5
        hover:bg-white dark:hover:bg-stone-700/90
      `}
      class:pointer-events-none={disabled}
      role={disabled ? "button" : "link"}
      aria-disabled={disabled}
    >
      <!-- Clean active indicator -->
      {#if isActive}
        <div
          class="bg-cacao-500 dark:bg-cacao-400 shadow-cacao-500/40 absolute top-1/2 left-0 h-10
            w-1 -translate-y-1/2
            rounded-r-full shadow-lg
          "
        ></div>
      {/if}

      <!-- Icon with subtle enhancement -->
      <div class="relative">
        <Icon class={`${iconClasses} ${isActive ? "drop-shadow-sm" : ""}`} />
      </div>

      <!-- Clean badge design -->
      {#if badgeVisible}
        <div
          class="absolute -top-1 -right-1 flex h-6 w-6 animate-pulse items-center justify-center
            rounded-full border-2
            border-white bg-red-500 text-xs
            font-bold text-white
            shadow-lg shadow-red-500/30 hover:scale-110
            hover:animate-none
            dark:border-stone-800 dark:bg-red-600
          "
        >
          {typeof badge === "number" && badge > 10 ? "10+" : badge}
        </div>
      {/if}
    </a>

    <!-- Clean tooltip -->
    <div
      class="pointer-events-none absolute top-1/2 left-16 z-50
        -translate-y-1/2 opacity-0 transition-all duration-300
        group-hover/tooltip:pointer-events-auto group-hover/tooltip:translate-x-2
        group-hover/tooltip:opacity-100
      "
    >
      <div
        class="relative rounded-lg border border-stone-700/50 bg-stone-800 px-3
          py-2 text-sm
          font-medium text-stone-100 shadow-xl
          shadow-stone-900/30 backdrop-blur-sm
          before:absolute
          before:top-1/2 before:left-0 before:-translate-x-1 before:-translate-y-1/2 before:border-4
          before:border-transparent before:border-r-stone-800 dark:bg-stone-900 dark:before:border-r-stone-900
        "
      >
        {name}
      </div>
    </div>
  </div>
{:else}
  <!-- Expanded sidebar -->
  <a
    {href}
    {target}
    {rel}
    class={`${baseClasses} ${stateClasses} rounded-lg
      hover:bg-stone-50/80 dark:hover:bg-stone-800/60
      ${isActive ? "bg-cacao-50/90 dark:bg-cacao-900/70 border-cacao-500 dark:border-cacao-400 border-l-2" : ""}
      border
      border-transparent hover:translate-x-1 hover:border-stone-200/50 dark:hover:border-stone-700/50
    `}
    class:pointer-events-none={disabled}
    role={disabled ? "button" : "link"}
    aria-disabled={disabled}
  >
    <!-- Clean active indicator -->
    {#if isActive}
      <div
        class="bg-cacao-500 dark:bg-cacao-400 shadow-cacao-500/30 absolute top-1/2 left-0 h-8
          w-1 -translate-y-1/2
          rounded-r-full shadow-md
        "
      ></div>
    {/if}

    <div class="relative">
      <Icon class={`${iconClasses} ${isActive ? "drop-shadow-sm" : ""}`} />
    </div>

    <div class="flex min-w-0 flex-1 items-center justify-between">
      <H3>
        {name}
      </H3>

      <!-- Clean badge for expanded state -->
      {#if badgeVisible}
        <div
          class="ml-2 flex h-6 min-w-[24px] animate-pulse items-center justify-center
            rounded-full border
            border-red-400/20 bg-red-500 px-2 text-xs
            font-bold text-white
            shadow-lg shadow-red-500/30
            hover:animate-none dark:bg-red-600
          "
        >
          {typeof badge === "number" && badge > 10 ? "10+" : badge}
        </div>
      {/if}

      <!-- External link indicator -->
      {#if external}
        <div
          class="ml-2 flex h-4 w-4 items-center justify-center opacity-60 transition-opacity hover:opacity-100"
        >
          <SquareArrowUpRight />
        </div>
      {/if}
    </div>
  </a>
{/if}
