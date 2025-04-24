<script lang="ts">
  import Label from "$lib/components/typography/Label.svelte";
  import { Eye, EyeClosed } from "lucide-svelte";

  let {
    placeholder = "Edit here",
    name = "name",
    labelName = name,
    value = $bindable(),
    disabled = $bindable(),
    type = "text",
  }: {
    placeholder: string;
    name: string;
    value: string | number | boolean | undefined;
    labelName?: string;
    disabled?: boolean;
    type?:
      | "text"
      | "number"
      | "textarea"
      | "password"
      | "email"
      | "checkbox"
      | "date"
      | string;
  } = $props();

  let showPassword = $state(false);

  const baseStyle =
    "w-full rounded-2xl bg-white dark:bg-stone-950 border border-stone-300 dark:border-stone-700 px-4 py-2 text-base text-stone-900 dark:text-stone-100 placeholder-stone-400 transition-all shadow-sm focus:shadow-md focus:outline-none focus:border-cacao-500 focus:ring-2 focus:ring-cacao-500/20 disabled:opacity-60 disabled:cursor-not-allowed";
</script>

<div class="relative space-y-1">
  <Label>{labelName}</Label>

  {#if type === "text"}
    <input
      {name}
      type="text"
      bind:value
      {disabled}
      class={baseStyle}
      {placeholder}
    />
  {:else if type === "textarea"}
    <textarea
      {name}
      rows="3"
      bind:value
      {disabled}
      class={baseStyle + " resize-none"}
      {placeholder}
    ></textarea>
  {:else if type === "number"}
    <input
      type="number"
      {placeholder}
      {name}
      {disabled}
      bind:value
      class={baseStyle}
    />
  {:else if type === "password"}
    <input
      type={showPassword ? "text" : "password"}
      {placeholder}
      {name}
      bind:value
      {disabled}
      class={baseStyle}
    />
    <button
      type="button"
      class="absolute top-[2.65rem] right-3 -translate-y-1/2 transform text-stone-500 dark:text-stone-300"
      onclick={() => (showPassword = !showPassword)}
      tabindex="-1"
    >
      {#if showPassword}
        <Eye class="h-5 w-5" />
      {:else}
        <EyeClosed class="h-5 w-5" />
      {/if}
    </button>
  {:else if type === "email"}
    <input
      type="email"
      {placeholder}
      {name}
      bind:value
      {disabled}
      class={baseStyle}
    />
  {:else if type === "date"}
    <input
      type="date"
      {placeholder}
      {name}
      bind:value
      {disabled}
      class={baseStyle}
    />
  {/if}
</div>
