<script lang="ts">
  import { Loader2 } from "lucide-svelte";
  import { isLoading } from "$lib/stores";
  import type { Component, Snippet } from "svelte";
  import type { MouseEventHandler } from "svelte/elements";

  type ButtonVariant =
    | "primary"
    | "secondary"
    | "danger"
    | "ghost"
    | "link"
    | "outline";

  type ButtonSize = "xs" | "sm" | "md" | "lg" | "xl";
  type ButtonType = "button" | "submit" | "reset" | undefined;

  interface Props {
    variant: ButtonVariant;
    size?: ButtonSize;
    type?: ButtonType;
    href?: string | undefined;
    formaction?: string | undefined;
    styling?: string;
    disable?: boolean;
    Icon: Component | undefined;
    iconPosition?: "left" | "right";
    fullWidth?: boolean;
    rounded?: boolean;
    confirmText?: string | undefined;
    confirmTitle?: string | undefined;
    customColors?: string | undefined;
    onclick?: MouseEventHandler<HTMLButtonElement> | undefined;
    children?: Snippet;
  }

  let {
    variant = "primary",
    size = "md",
    type = "button",
    href = undefined,
    formaction = undefined,
    styling = "",
    disable = false,
    Icon = undefined,
    iconPosition = "left",
    fullWidth = false,
    rounded = false,
    confirmText = undefined,
    confirmTitle = undefined,
    onclick = undefined,
    children,
  }: Props = $props();

  const isLink = $derived(!!href);
  let disabled = $derived($isLoading || disable);
  let showConfirmDialog = $state(false);

  function handleClick(event: any) {
    if (variant === "danger" && (confirmText || confirmTitle)) {
      event.preventDefault();
      showConfirmDialog = true;
    }
  }

  const sizeClasses = {
    xs: "px-2 py-1 text-tiny",
    sm: "px-3 py-1.5 text-sm",
    md: "px-4 py-2 text-[15px]",
    lg: "px-5 py-2.5 text-base md:px-6",
    xl: "px-6 py-3 text-lg md:px-8",
  };

  const baseClasses =
    "flex items-center justify-center rounded-lg transition-all duration-150 ease-in-out font-medium select-none focus:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-cacao-400 disabled:cursor-not-allowed disabled:opacity-50 backdrop-blur-md";

  const variantClasses = {
    primary:
      "bg-gradient-to-br from-white/80 transition-colors to-stone-100/80 text-stone-900 shadow-sm ring-1 ring-stone-300/40 hover:from-white hover:to-stone-200 dark:from-stone-900/70 dark:to-stone-800/70 dark:text-white dark:hover:from-stone-800 dark:hover:to-stone-700",
    secondary:
      "bg-white/70 text-stone-700 ring-1 ring-stone-300 hover:bg-white shadow-sm dark:bg-stone-900/80 dark:text-stone-200 hover:dark:bg-stone-800 dark:ring-stone-600/40",
    danger:
      "bg-gradient-to-br from-red-100/80 transition-colors to-red-200/80 text-red-800 hover:from-red-200 hover:to-red-300 ring-1 ring-red-300/40 shadow-sm dark:from-red-700/70 dark:to-red-800/70 dark:text-white dark:hover:from-red-600 dark:hover:to-red-700",
    ghost:
      "text-stone-600 dark:text-stone-400 hover:bg-stone-100/60 dark:hover:bg-stone-800/60",
    link: "text-cacao-600 underline hover:text-cacao-800 p-0 ring-0 dark:text-cacao-300 dark:hover:text-cacao-100",
    outline:
      "bg-transparent text-stone-800 ring-1 ring-stone-300 hover:bg-stone-50 dark:text-stone-200 dark:ring-stone-600/50 dark:hover:bg-stone-800",
  };

  const shapeClasses = $derived(rounded ? "rounded-full" : "rounded-lg");
  const widthClasses = $derived(fullWidth ? "w-full" : "");

  const allClasses = $derived(
    [
      baseClasses,
      variantClasses[variant],
      sizeClasses[size],
      shapeClasses,
      widthClasses,
      styling,
    ].join(" "),
  );
</script>

{#if isLink}
  <a {href} class={allClasses} aria-disabled={disabled}>
    {#if $isLoading}
      <Loader2 class="mr-2 size-4 animate-spin" />
    {:else if Icon && iconPosition === "left"}
      <Icon class="mr-2 size-4" />
    {/if}

    {#if $isLoading}Loading...
    {:else}
      {@render children?.()}
    {/if}

    {#if Icon && iconPosition === "right"}
      <Icon class="ml-2 size-4" />
    {/if}
  </a>
{:else}
  <button
    {type}
    {formaction}
    {disabled}
    class={allClasses}
    onclick={variant === "danger" ? handleClick : onclick}
  >
    {#if $isLoading}
      <Loader2 class="mr-2 size-4 animate-spin" />
    {:else if Icon && iconPosition === "left"}
      <Icon class="mr-2 size-4" />
    {/if}

    {#if $isLoading}Loading...
    {:else}
      {@render children?.()}
    {/if}

    {#if Icon && iconPosition === "right"}
      <Icon class="ml-2 size-4" />
    {/if}
  </button>
{/if}

{#if showConfirmDialog}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/30 backdrop-blur-sm"
  >
    <div
      class="w-full max-w-sm rounded-xl bg-white/90 p-6 shadow-sm ring-1 ring-stone-300/40 dark:bg-stone-900/90 dark:ring-stone-600/50"
    >
      <h3 class="text-lg font-semibold text-stone-800 dark:text-stone-200">
        {confirmTitle || "Are you sure?"}
      </h3>
      <p class="mt-2 text-sm text-stone-600 dark:text-stone-400">
        {confirmText || "This action cannot be undone."}
      </p>
      <div class="mt-5 flex justify-end gap-2">
        <button
          type="button"
          class="rounded-lg bg-white px-4 py-2 text-stone-700 ring-1 ring-stone-300 transition-all hover:bg-stone-50 dark:bg-stone-800 dark:text-stone-200 dark:hover:bg-stone-700"
          onclick={() => (showConfirmDialog = false)}
        >
          Cancel
        </button>
        <button
          type="submit"
          class="rounded-lg bg-gradient-to-br from-red-500 to-red-600 px-4 py-2 text-white ring-1 ring-red-300 transition-all hover:from-red-500 hover:to-red-700 focus:ring focus:ring-red-400 focus:ring-offset-2"
          {formaction}
        >
          Confirm
        </button>
      </div>
    </div>
  </div>
{/if}
