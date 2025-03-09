<script lang="ts">
  import Label from "$lib/components/typography/Label.svelte";

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
      | string;
  } = $props();

  const styling =
    "mt-2 w-full rounded-md p-3 text-lg shadow-sm dark:border-stone-700 dark:bg-stone-950 disabled:text-stone-500 ring ring-stone-200 dark:ring-stone-800";
</script>

<Label>{labelName}</Label>
{#if type === "text"}
  <input
    {name}
    type="text"
    bind:value
    {disabled}
    class="{styling} focus:border-cacao-500 focus:ring-cacao-500"
    {placeholder}
  />
{:else if type === "textarea"}
  <textarea
    {name}
    rows="5"
    bind:value
    {disabled}
    class="{styling} focus:border-cacao-500 focus:ring-cacao-500"
    {placeholder}
  ></textarea>
{:else if type === "number"}
  <input
    type="number"
    {placeholder}
    {name}
    {disabled}
    bind:value
    class="{styling} focus:border-cacao-500 focus:ring-cacao-500"
  />
{:else if type === "password"}
  <input
    type="password"
    {placeholder}
    {name}
    bind:value
    {disabled}
    class="{styling} focus:border-cacao-500 focus:ring-cacao-500"
  />
{:else if type === "email"}
  <input
    type="email"
    {placeholder}
    {name}
    bind:value
    {disabled}
    class="{styling} focus:border-cacao-500 focus:ring-cacao-500"
  />
{/if}
