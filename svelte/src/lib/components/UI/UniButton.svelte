<script lang="ts">
  import { isLoading } from "$lib/stores";
  import type { ComponentType, Snippet } from "svelte";
  import type { MouseEventHandler } from "svelte/elements";
  import ConfirmDialogue from "./ConfirmDialogue.svelte";
  import { Headline } from "../typography";

  type ButtonVariant = "primary" | "danger" | "prominent";

  type ButtonType = "button" | "submit" | "reset" | undefined;

  interface Props {
    variant?: ButtonVariant;
    type?: ButtonType;
    href?: string | undefined;
    formaction?: string | undefined;
    styling?: string;
    disable?: boolean;
    Icon?: ComponentType | undefined;
    confirmText?: string | undefined;
    confirmTitle?: string | undefined;
    onclick?: MouseEventHandler<HTMLButtonElement> | undefined;
    children?: Snippet;
    iconOnly?: boolean;
  }

  let {
    variant = "primary",
    type = "button",
    href = undefined,
    formaction = undefined,
    styling = "",
    disable = false,
    Icon = undefined,
    confirmText = undefined,
    confirmTitle = undefined,
    onclick = undefined,
    children,
    iconOnly = true,
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

  const baseClasses = `
  flex items-center justify-center flex-1 p-2
  rounded-full font-medium focus-visible:outline-none
  disabled:opacity-50 disabled:pointer-events-none
  backdrop-blur-sm gap-2
`;

  const variantClasses = {
    primary: "hover:bg-stone-100 dark:hover:bg-stone-700",
    danger:
      "text-red-600 dark:text-red-50 hover:bg-red-100 dark:hover:bg-red-800",
    prominent: "hover:bg-accent/80 bg-accent text-white",
  };

  const allClasses = $derived(
    [baseClasses, variantClasses[variant], styling].join(" "),
  );
</script>

{#if isLink}
  <a {href} class={allClasses} aria-disabled={disabled}>
    {#if Icon}
      <Icon class="size-5" />
    {/if}
    {#if !iconOnly}
      <Headline>
        {@render children?.()}
      </Headline>
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
    {#if Icon}
      <Icon class="size-5" />
    {/if}

    {#if !iconOnly}
      <Headline>
        {@render children?.()}
      </Headline>
    {/if}
  </button>
{/if}

{#if showConfirmDialog}
  <ConfirmDialogue {formaction} bind:showConfirmDialog />
{/if}
