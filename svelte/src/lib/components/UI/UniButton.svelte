<script lang="ts">
  import { Loader2 } from "lucide-svelte";
  import { isLoading } from "$lib/stores";
  import type { Snippet } from "svelte";
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
    Icon: ConstructorOfATypedSvelteComponent | undefined;
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
    customColors = undefined,
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
    xs: "px-2 py-1 text-xs",
    sm: "px-2.5 py-1.5 text-sm",
    md: "px-3 py-2 text-sm md:px-4 md:text-base",
    lg: "px-4 py-2.5 text-base md:px-6",
    xl: "px-5 py-3 text-lg md:px-8",
  };

  const baseClasses =
    "flex items-center justify-center rounded-lg ring transition-all focus:ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50";

  const variantClasses = {
    primary:
      customColors ||
      "from-cacao-500 to-cacao-600 text-cacao-50 dark:from-stone-800 dark:to-stone-900 dark:text-cacao-100 hover:to-cacao-700 focus:ring-cacao-500 ring-stone-200 dark:ring-stone-800 dark:hover:ring-stone-700 dark:hover:to-stone-950 bg-gradient-to-br",
    secondary:
      customColors ||
      "text-stone-700 from-stone-50 to-stone-100 hover:to-stone-200 ring-stone-300 bg-gradient-to-bl",
    danger:
      customColors ||
      "from-red-500 to-red-600 text-white hover:from-red-500 hover:to-red-700 dark:from-red-500 dark:to-red-600 dark:hover:from-red-500 dark:hover:to-red-700 focus:ring-red-400 bg-gradient-to-br",
    ghost:
      customColors ||
      "text-stone-600 dark:text-stone-400 hover:bg-stone-100 dark:hover:bg-stone-800 ring-transparent",
    link:
      customColors ||
      "text-cacao-600 dark:text-cacao-400 underline hover:text-cacao-700 dark:hover:text-cacao-300 p-0 ring-transparent",
    outline:
      customColors ||
      "text-stone-700 dark:text-stone-300 ring-1 ring-stone-300 dark:ring-stone-700 hover:bg-stone-50 dark:hover:bg-stone-800",
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
    class="fixed inset-0 z-50 flex items-center justify-center bg-stone-950/50 p-4"
  >
    <div
      class="w-full max-w-md rounded-xl bg-white p-6 shadow-xl dark:bg-stone-900"
    >
      <h3 class="text-xl font-semibold text-stone-800 dark:text-stone-200">
        {confirmTitle || "Confirm"}
      </h3>
      <p class="mt-2 text-stone-600 dark:text-stone-400">
        Are you sure you want to {"delete " + confirmText || "continue"}? This
        action cannot be undone.
      </p>
      <div class="mt-6 flex justify-end gap-3">
        <button
          type="button"
          class="rounded-lg bg-gradient-to-bl from-stone-50 to-stone-100 px-3 py-2 text-center text-stone-700 ring ring-stone-300 transition-colors hover:to-stone-200"
          onclick={() => (showConfirmDialog = false)}
        >
          Cancel
        </button>
        <button
          type="submit"
          class="rounded-lg bg-gradient-to-br from-red-500 to-red-600 px-3 py-2 text-center text-white ring transition-colors hover:from-red-500 hover:to-red-700 focus:ring focus:ring-red-400 focus:ring-offset-2 focus:outline-none dark:from-red-500 dark:to-red-600 dark:hover:from-red-500 dark:hover:to-red-700"
          {formaction}
        >
          Confirm
        </button>
      </div>
    </div>
  </div>
{/if}
