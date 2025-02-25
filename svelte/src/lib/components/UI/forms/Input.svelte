<script lang="ts">
  let {
    placeholder = "Edit here",
    name = "name",
    value = $bindable(),
    type = "text",
  }: {
    placeholder: string;
    name: string;
    value: string | number | boolean | undefined;
    type?: "text" | "number" | "textarea" | "password" | "email" | "checkbox";
  } = $props();

  const styling =
    "mt-2 w-full rounded-md p-3 text-lg shadow-sm dark:border-milk-700 dark:bg-milk-950";
</script>

{#if type === "text"}
  <input
    {name}
    type="text"
    bind:value
    class="{styling} focus:border-cacao-500 focus:ring-cacao-500"
    {placeholder}
  />
{:else if type === "textarea"}
  <textarea
    {name}
    rows="5"
    bind:value
    class="{styling} focus:border-cacao-500 focus:ring-cacao-500"
    {placeholder}
  ></textarea>
{:else if type === "number"}
  <input
    type="number"
    {placeholder}
    {name}
    bind:value
    class="{styling} focus:border-cacao-500 focus:ring-cacao-500"
  />
{:else if type === "password"}
  <input
    type="password"
    {placeholder}
    {name}
    bind:value
    class="{styling} focus:border-cacao-500 focus:ring-cacao-500"
  />
{:else if type === "email"}
  <input
    type="email"
    {placeholder}
    {name}
    bind:value
    class="{styling} focus:border-cacao-500 focus:ring-cacao-500"
  />
{/if}
