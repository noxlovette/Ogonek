<script lang="ts">
  import { isLoading } from "$lib/stores";
  import type { MouseEventHandler } from "svelte/elements";
  import ConfirmDialogue from "../ConfirmDialogue.svelte";
  import { Footnote, Headline } from "../../../typography";
  import { Trash2, X } from "@lucide/svelte";
  import type { Icon as IconType } from "@lucide/svelte";
  import tippy from "tippy.js";
  import type { Attachment } from "svelte/attachments";

  type ButtonVariant = "primary" | "danger" | "prominent";

  type ButtonType = "button" | "submit" | "reset" | undefined;

  interface Props {
    variant?: ButtonVariant;
    content: string;
    type?: ButtonType;
    href?: string | undefined;
    formaction?: string | undefined;
    styling?: string;
    disable?: boolean;
    Icon?: typeof IconType | undefined;
    fill?: boolean;
    shouldConfirm?: boolean | undefined;
    onclick?: MouseEventHandler<HTMLButtonElement> | undefined;
    ariaLabel?: string;
    description?: string;
    iconOnly?: boolean;
    dataCy?: string;
  }

  let {
    variant = "primary",
    type = "button",
    content,
    href = undefined,
    formaction = undefined,
    styling = "relative",
    disable = false,
    Icon = undefined,
    fill = false,
    shouldConfirm = false,
    onclick = undefined,
    ariaLabel = undefined,
    description = content,
    iconOnly = true,
    dataCy,
    ...rest
  }: Props = $props();

  const isLink = $derived(!!href);
  let disabled = $derived($isLoading || disable);
  let showConfirmDialog = $state(false);

  const uniqueId = `btn-${Math.random().toString(36).substr(2, 9)}`;

  function handleClick(event: MouseEvent) {
    if (variant === "danger" && shouldConfirm) {
      event.preventDefault();
      deleteClicked = !deleteClicked;
      showConfirmDialog = !showConfirmDialog;
    }
  }

  const baseClasses = `
  flex items-center transition-all duration-150 justify-center flex-1 p-2 md:p-3
  rounded-full font-medium focus-visible:outline-none
  disabled:opacity-20 disabled:pointer-events-none z-40 gap-2 
`;

  const variantClasses = {
    primary: "hover-default",
    danger:
      "text-rose-600 dark:text-rose-50 hover:bg-rose-100 dark:hover:bg-rose-800",
    prominent: "hover:bg-accent/60 bg-accent dark:bg-accent/90 text-white",
  };

  const allClasses = $derived(
    [baseClasses, variantClasses[variant], styling].join(" "),
  );

  let deleteClicked = $state(false);

  function tooltip(content?: string): Attachment | undefined {
    if (!iconOnly) return undefined;

    return (element) => {
      const tooltip = tippy(element, {
        content,
        arrow: false,
        duration: 0,
        delay: [200, 200],
      });
      return tooltip.destroy;
    };
  }
</script>

{#if isLink}
  <a
    {@attach tooltip(description)}
    {href}
    class={allClasses}
    aria-disabled={disabled}
    data-cy={dataCy}
    aria-label={ariaLabel}
    aria-describedby={description ? `${uniqueId}-desc` : undefined}
    {...rest}
    role="button"
  >
    {#if Icon}
      <Icon
        color={fill ? "#df7055" : "currentColor"}
        fill={fill ? "#df7055" : "#00000000"}
        class="size-5"
      />
    {/if}
    {#if !iconOnly}
      <Headline>
        {content}
      </Headline>
    {/if}
  </a>
{:else}
  <button
    {@attach tooltip(description)}
    {type}
    {formaction}
    {disabled}
    data-cy={dataCy}
    id={uniqueId}
    aria-label={ariaLabel}
    aria-describedby={description ? `${uniqueId}-desc` : undefined}
    class={allClasses}
    onclick={variant === "danger" ? handleClick : onclick}
    {...rest}
  >
    {#if Icon}
      <Icon
        color={fill ? "#df7055" : "currentColor"}
        fill={fill ? "#df7055" : "#00000000"}
        class="size-5"
      />
    {:else if variant == "danger" && !Icon}
      {#if !deleteClicked}
        <Trash2 />
      {:else}
        <X />
      {/if}
    {/if}

    {#if !iconOnly}
      <Footnote>
        {content}
      </Footnote>
    {/if}
  </button>
{/if}
{#if description}
  <div id="{uniqueId}-desc" class="sr-only">
    {description}
  </div>
{/if}
{#if showConfirmDialog}
  <ConfirmDialogue {formaction} bind:showConfirmDialog />
{/if}
