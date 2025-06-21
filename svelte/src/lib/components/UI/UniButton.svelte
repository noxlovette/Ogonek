<script lang="ts">
  import { Loader2 } from "lucide-svelte";
  import { isLoading } from "$lib/stores";
  import type { ComponentType, Snippet } from "svelte";
  import type { MouseEventHandler } from "svelte/elements";
  import ModalBackGround from "./forms/ModalBackGround.svelte";

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
    Icon: ComponentType | undefined;
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

  function handleClick(event: MouseEvent) {
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

  const baseClasses = `
  flex items-center justify-center
  hover:scale-[1.02]
  rounded-full font-medium select-none
  transition-all duration-200 ease-out
  active:scale-[0.97] focus-visible:outline-none
  ring-offset-2
  focus-visible:ring-2 focus-visible:ring-offset-3 focus-visible:ring-cacao-400
  disabled:opacity-50 disabled:pointer-events-none
  backdrop-blur-sm
`;

  const variantClasses = {
    primary: `
    bg-stone-100/60 text-stone-900 ring-1 ring-stone-200 ring-offset-stone-100 dark:ring-offset-stone-800 shadow-md 
    hover:bg-stone-200/40
    dark:bg-stone-800 dark:text-white dark:ring-stone-900 dark:hover:bg-stone-700
  `,
    secondary: `
    bg-white text-stone-700 ring-1 ring-stone-200 shadow-md dark:ring-offset-stone-800
    hover:bg-stone-50
    dark:bg-stone-900 dark:text-stone-200 dark:ring-stone-700 dark:hover:bg-stone-800
  `,
    danger: `
    bg-red-100 text-red-800 ring-1 ring-red-300 ring-offset-red-200 shadow-md
    hover:bg-red-200
    dark:bg-red-700 dark:text-white dark:ring-red-600 dark:ring-offset-red-500 dark:hover:bg-red-600
  `,
    ghost: `
    bg-transparent text-stone-600
    hover:bg-stone-100
    dark:text-stone-400 dark:hover:bg-stone-800
  `,
    link: `
    text-cacao-600 underline underline-offset-2 ring-0 p-0
    hover:text-cacao-800
    dark:text-cacao-300 dark:hover:text-cacao-100
  `,
    outline: `
    bg-transparent text-stone-800 ring-1 ring-stone-300
    hover:bg-stone-50
    dark:text-stone-200 dark:ring-stone-600 dark:hover:bg-stone-800
  `,
  };

  const widthClasses = $derived(fullWidth ? "w-full" : "");

  const allClasses = $derived(
    [
      baseClasses,
      variantClasses[variant],
      sizeClasses[size],
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
  <ModalBackGround>
    <h3 class="text-lg font-semibold text-stone-800 dark:text-stone-200">
      {confirmTitle || "Are you sure?"}
    </h3>
    <p class="mt-2 text-sm text-stone-600 dark:text-stone-400">
      {confirmText || "This action cannot be undone."}
    </p>
    <div class="mt-5 flex justify-end gap-2">
      <button
        type="button"
        class="rounded-lg bg-white px-4 py-2 text-stone-700/30 ring-1 ring-stone-300 transition-all hover:bg-stone-50 dark:bg-stone-800/30 dark:text-stone-200 dark:hover:bg-stone-700"
        onclick={() => (showConfirmDialog = false)}
      >
        Cancel
      </button>
      <button
        type="submit"
        class="rounded-lg bg-gradient-to-br from-red-500 to-red-600 px-4 py-2 text-stone-50 ring-1 ring-red-300 transition-all hover:from-red-500 hover:to-red-700 focus:ring focus:ring-red-400 focus:ring-offset-2"
        {formaction}
      >
        Confirm
      </button>
    </div>
  </ModalBackGround>
{/if}
