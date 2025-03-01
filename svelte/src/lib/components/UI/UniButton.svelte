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
    children: Snippet;
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

  // Simple function to handle showing the confirmation dialog for danger buttons
  function handleClick(event) {
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
      "from-cacao-500 to-cacao-600 text-cacao-50 dark:from-milk-800 dark:to-milk-900 dark:text-cacao-100 hover:to-cacao-700 focus:ring-cacao-500 ring-milk-200 dark:ring-milk-800 dark:hover:ring-milk-700 dark:hover:to-milk-950 bg-gradient-to-br",
    secondary:
      customColors ||
      "text-milk-700 from-milk-50 to-milk-100 hover:to-milk-200 ring-milk-300 bg-gradient-to-bl",
    danger:
      customColors ||
      "from-red-500 to-red-600 text-white hover:from-red-500 hover:to-red-700 dark:from-red-500 dark:to-red-600 dark:hover:from-red-500 dark:hover:to-red-700 focus:ring-red-400 bg-gradient-to-br",
    ghost:
      customColors ||
      "text-milk-600 dark:text-milk-400 hover:bg-milk-100 dark:hover:bg-milk-800 ring-transparent",
    link:
      customColors ||
      "text-cacao-600 dark:text-cacao-400 underline hover:text-cacao-700 dark:hover:text-cacao-300 p-0 ring-transparent",
    outline:
      customColors ||
      "text-milk-700 dark:text-milk-300 ring-1 ring-milk-300 dark:ring-milk-700 hover:bg-milk-50 dark:hover:bg-milk-800",
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
      {@render children()}
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
      {@render children()}
    {/if}

    {#if Icon && iconPosition === "right"}
      <Icon class="ml-2 size-4" />
    {/if}
  </button>
{/if}

{#if showConfirmDialog}
  <div
    class="bg-milk-950/50 fixed inset-0 z-50 flex items-center justify-center p-4"
  >
    <div
      class="dark:bg-milk-900 w-full max-w-md rounded-xl bg-white p-6 shadow-xl"
    >
      <h3 class="text-milk-800 dark:text-milk-200 text-xl font-semibold">
        {confirmTitle || "Confirm"}
      </h3>
      <p class="text-milk-600 dark:text-milk-400 mt-2">
        Are you sure you want to {"delete " + confirmText || "continue"}? This
        action cannot be undone.
      </p>
      <div class="mt-6 flex justify-end gap-3">
        <button
          type="button"
          class="text-milk-700 from-milk-50 to-milk-100 hover:to-milk-200 ring-milk-300 rounded-lg bg-gradient-to-bl px-3 py-2 text-center ring transition-colors"
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
