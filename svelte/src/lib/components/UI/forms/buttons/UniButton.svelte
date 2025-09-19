<script lang="ts">
  import { isLoading } from "$lib/stores";
  import type { ComponentType, Snippet } from "svelte";
  import type { MouseEventHandler } from "svelte/elements";
  import ConfirmDialogue from "../ConfirmDialogue.svelte";
  import { Footnote, Headline } from "../../../typography";
  import { Ban, Trash2, X } from "lucide-svelte";

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
    shouldConfirm?: boolean | undefined;
    onclick?: MouseEventHandler<HTMLButtonElement> | undefined;
    children?: Snippet;
    ariaLabel?: string;
    iconOnly?: boolean;
  }

  let {
    variant = "primary",
    type = "button",
    href = undefined,
    formaction = undefined,
    styling = "relative",
    disable = false,
    Icon = undefined,
    shouldConfirm = false,
    onclick = undefined,
    children,
    ariaLabel = undefined,
    iconOnly = true,
  }: Props = $props();

  const isLink = $derived(!!href);
  let disabled = $derived($isLoading || disable);
  let showConfirmDialog = $state(false);

  function handleClick(event: MouseEvent) {
    if (variant === "danger" && shouldConfirm) {
      event.preventDefault();
      deleteClicked = !deleteClicked;
      showConfirmDialog = !showConfirmDialog;
    }
  }

  const baseClasses = `
  flex items-center  justify-center flex-1 p-2 md:p-3
  rounded-full font-medium focus-visible:outline-none
  disabled:opacity-50 disabled:pointer-events-none z-40 gap-2
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

  let deleteClicked = $state(false);
</script>

{#if isLink}
  <a {href} class={allClasses} aria-disabled={disabled} aria-label={ariaLabel}>
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
    id="btn"
    aria-label={ariaLabel}
    class={allClasses}
    onclick={variant === "danger" ? handleClick : onclick}
  >
    {#if Icon && variant !== "danger"}
      <Icon class="size-5" />
    {:else if variant == "danger"}
      {#if !deleteClicked}
        <Trash2 />
      {:else}
        <X />
      {/if}
    {/if}

    {#if !iconOnly}
      <Footnote>
        {@render children?.()}
      </Footnote>
    {/if}
  </button>
{/if}

{#if showConfirmDialog}
  <ConfirmDialogue {formaction} bind:showConfirmDialog />
{/if}
