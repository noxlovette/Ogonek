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

  // Modern glass morphism and micro-interaction classes
  const baseClasses =
    "group relative flex items-center gap-3 active:scale-[0.98] hover:scale-[1.03] px-3 py-2.5 text-sm font-medium transition-all duration-200 ease-out select-none";

  const stateClasses = $derived(() => {
    if (disabled) return "opacity-50 cursor-not-allowed";
    return "text-stone-700 dark:text-stone-300 active:scale-[0.98]";
  });

  const iconClasses = $derived(() => {
    const base = "transition-all duration-200 flex-shrink-0";
    if ($sidebar) return `${base} size-5`;
    return `${base} size-5 ${isActive ? "text-cacao-600 dark:text-cacao-400" : "text-stone-500 dark:text-stone-400"}`;
  });

  const badgeVisible = $derived(badge && badge != 0 ? true : false);
</script>

<!-- Tooltip for collapsed sidebar -->
{#if $sidebar && !rightBar}
  <div class="group/tooltip relative">
    <a
      {href}
      {target}
      {rel}
      aria-label={name}
      class={`${baseClasses} ${stateClasses} min-h-[44px] min-w-[44px] justify-center`}
      class:pointer-events-none={disabled}
      role={disabled ? "button" : "link"}
      aria-disabled={disabled}
    >
      <!-- Active indicator line -->
      {#if isActive}
        <div
          class="bg-cacao-500 dark:bg-cacao-400 absolute top-1/2 left-0 h-6 w-0.5 -translate-y-1/2 rounded-r-full"
        ></div>
      {/if}

      <Icon class={iconClasses} />

      <!-- Badge for collapsed state -->
      {#if badgeVisible}
        <div
          class="absolute -top-1 -right-1 flex h-5 w-5 items-center justify-center rounded-full bg-red-500 text-xs font-bold text-white"
        >
          {typeof badge === "number" && badge > 10 ? "10+" : badge}
        </div>
      {/if}
    </a>
  </div>
{:else}
  <a
    {href}
    {target}
    {rel}
    class={`${baseClasses} ${stateClasses} overflow-hidden`}
    class:pointer-events-none={disabled}
    role={disabled ? "button" : "link"}
    aria-disabled={disabled}
  >
    <!-- Active indicator line -->
    {#if isActive}
      <div
        class="bg-cacao-500 dark:bg-cacao-400 absolute top-1/2 left-0 h-6 w-0.5 -translate-y-1/2 rounded-r-full"
      ></div>
    {/if}

    <Icon class={iconClasses} />

    <div class="flex min-w-0 flex-1 items-center justify-between">
      <H3>
        {name}
      </H3>

      <!-- Badge for expanded state -->
      {#if badgeVisible}
        <div
          class="ml-2 flex h-5 min-w-[20px] items-center justify-center rounded-full bg-red-500 px-1.5 text-xs font-bold text-white"
        >
          {typeof badge === "number" && badge > 10 ? "10+" : badge}
        </div>
      {/if}

      <!-- External link indicator -->
      {#if external}
        <div class="ml-2 flex h-4 w-4 items-center justify-center">
          <SquareArrowUpRight />
        </div>
      {/if}
    </div>
  </a>
{/if}
