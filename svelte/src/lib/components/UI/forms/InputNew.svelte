<script lang="ts">
  import { Body } from "$lib/components/typography";
  import Caption1 from "$lib/components/typography/Caption1.svelte";
  import { BadgeCheck, BadgeQuestionMark } from "@lucide/svelte";
  import type { Attachment } from "svelte/attachments";

  import type { ChangeEventHandler } from "svelte/elements";
  import type { HTMLInputAttributes } from "svelte/elements";
  import tippy from "tippy.js";

  let {
    placeholder = "Менять тут",
    name,
    labelName = name.charAt(0).toUpperCase() + name.slice(1),
    value = $bindable(),
    disabled = $bindable(),
    invalid = false,
    invalidDescription,
    showLabel = true,
    item,
    onchange,
    dataCy,
    clarification,
    verified,
    ...rest
  }: {
    placeholder?: string;
    name: string;
    value?: string | number | boolean | null;
    labelName?: string;
    invalid?: boolean;
    invalidDescription?: string;
    disabled?: boolean;
    showLabel?: boolean;
    onchange?: ChangeEventHandler<HTMLInputElement>;
    item?: Assignable | null;
    dataCy?: string;
    clarification?: string;
    verified?: boolean;
  } & HTMLInputAttributes = $props();

  type Assignable = { assignee?: string | null };
  const baseStyle = $derived(
    `input-default ${invalid ? "ring-error text-red-500" : ""}`,
  );

  function tooltip(content?: string): Attachment | undefined {
    if (verified) return undefined;

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

<div
  class="gap-default relative grid grid-cols-2 items-center justify-between rounded-xl p-2.5"
>
  <div class="flex flex-col">
    {#if showLabel}
      <Body>{labelName}</Body>
    {/if}
    {#if clarification}
      <Caption1>
        {clarification}
      </Caption1>
    {/if}
  </div>
  <div>
    <input
      {...rest}
      {name}
      bind:value
      {disabled}
      data-cy={dataCy}
      class={baseStyle}
      {placeholder}
      aria-label={showLabel ? undefined : labelName}
      aria-invalid={invalid}
      aria-describedby={invalid && invalidDescription
        ? `${name}-error`
        : undefined}
    />
    {#if invalidDescription && invalid}
      <Caption1 styling="absolute right-5 top-6" override="text-red-400">
        {invalidDescription}
      </Caption1>
    {/if}

    <button
      {@attach tooltip("Нажмите, чтобы отправить подтверждение еще раз")}
      class="absolute top-5 right-4 flex items-end"
      formaction="?/resendEmailVerification"
      type="submit"
    >
      {#if verified == true}
        <BadgeCheck class="text-emerald-400" />
      {:else if verified == false}
        <BadgeQuestionMark class="text-red-400" />
      {/if}
    </button>
  </div>
</div>
