<script lang="ts">
  import type { ComponentType } from "svelte";
  import { fade, fly } from "svelte/transition";
  import { cubicOut } from "svelte/easing";

  interface ToolbarAction {
    id: string;
    icon: ComponentType;
    label: string;
    action: () => void | Promise<void>;
    disabled?: boolean;
    variant?: "default" | "primary" | "danger";
    badge?: number;
    shortcut?: string;
  }

  interface Props {
    title?: string;
    subtitle?: string;
    actions: ToolbarAction[];
    showDivider?: boolean;
    compact?: boolean;
  }

  let {
    title = "",
    subtitle = "",
    actions = [],
    showDivider = true,
    compact = false,
  }: Props = $props();

  let isVisible = $state(true);
  let lastScrollY = $state(0);

  // Auto-hide on scroll down (iOS Safari style)
  if (typeof window !== "undefined") {
    window.addEventListener("scroll", () => {
      const currentScrollY = window.scrollY;
      isVisible = currentScrollY < lastScrollY || currentScrollY < 10;
      lastScrollY = currentScrollY;
    });
  }

  function getVariantClasses(variant: string = "default"): string {
    const variants = {
      default:
        "text-stone-700 hover:bg-stone-100 active:bg-stone-200 dark:text-stone-300 dark:hover:bg-stone-800 dark:active:bg-stone-700",
      primary:
        "text-blue-600 hover:bg-blue-50 active:bg-blue-100 dark:text-blue-400 dark:hover:bg-blue-950 dark:active:bg-blue-900",
      danger:
        "text-red-600 hover:bg-red-50 active:bg-red-100 dark:text-red-400 dark:hover:bg-red-950 dark:active:bg-red-900",
    };
    return variants[variant as keyof typeof variants] || variants.default;
  }

  function handleAction(action: ToolbarAction) {
    if (action.disabled) return;
    action.action();
  }

  function handleKeydown(event: KeyboardEvent, action: ToolbarAction) {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      handleAction(action);
    }
  }
</script>

{#if isVisible}
  <header
    class="sticky top-0 z-40 w-full border-b border-stone-200/50 bg-white/80 backdrop-blur-xl dark:border-stone-800/50 dark:bg-stone-950/80"
    class:h-12={compact}
    class:h-16={!compact}
    transition:fly={{ y: -100, duration: 300, easing: cubicOut }}
  >
    <div class="flex h-full items-center justify-between px-4 md:px-6">
      <!-- Title Section -->
      <div class="flex min-w-0 flex-1 flex-col justify-center">
        {#if title}
          <h1
            class="truncate text-lg font-semibold text-stone-900 dark:text-stone-100"
          >
            {title}
          </h1>
        {/if}
        {#if subtitle}
          <p class="truncate text-sm text-stone-500 dark:text-stone-400">
            {subtitle}
          </p>
        {/if}
      </div>

      <!-- Actions Section -->
      <div class="flex items-center gap-1">
        {#each actions as action (action.id)}
          <button
            type="button"
            class="relative flex items-center justify-center rounded-lg p-2 transition-all duration-150 ease-out {getVariantClasses(
              action.variant,
            )} focus:ring-2 focus:ring-blue-500/20 focus:ring-offset-1 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50 dark:focus:ring-offset-stone-900"
            class:cursor-not-allowed={action.disabled}
            disabled={action.disabled}
            title="{action.label}{action.shortcut
              ? ` (${action.shortcut})`
              : ''}"
            onclick={() => handleAction(action)}
            onkeydown={(e) => handleKeydown(e, action)}
            aria-label={action.label}
          >
            <svelte:component this={action.icon} size={18} />

            {#if action.badge && action.badge > 0}
              <span
                class="absolute -top-1 -right-1 flex h-5 w-5 items-center justify-center rounded-full bg-red-500 text-xs font-medium text-white"
                transition:fade={{ duration: 200 }}
              >
                {action.badge > 99 ? "99+" : action.badge}
              </span>
            {/if}
          </button>
        {/each}
      </div>
    </div>

    {#if showDivider}
      <div
        class="absolute right-0 bottom-0 left-0 h-px bg-gradient-to-r from-transparent via-stone-200 to-transparent dark:via-stone-800"
      ></div>
    {/if}
  </header>
{/if}

<style>
  /* Add subtle text shadow for better legibility on backdrop blur */
  h1 {
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  }

  /* Smooth hover animations */
  button {
    transform: translateZ(0); /* Force hardware acceleration */
  }

  button:hover:not(:disabled) {
    transform: translateY(-0.5px);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  button:active:not(:disabled) {
    transform: translateY(0);
    transition-duration: 50ms;
  }
</style>
