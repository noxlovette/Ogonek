<script lang="ts">
  import Label from "$lib/components/typography/Label.svelte";
  import { m } from "$lib/paraglide/messages";
  import { assigneeStore, studentStore, user } from "$lib/stores";
  import { Eye, EyeClosed } from "lucide-svelte";

  let {
    placeholder = "Edit here",
    name,
    labelName = name,
    value = $bindable(),
    disabled = $bindable(),
    ref,
    showLabel = true,
    item,
    type = "text",
  }: {
    placeholder?: string;
    name: string;
    value?: string | number | boolean | null;
    labelName?: string;
    ref?: HTMLInputElement;
    disabled?: boolean;
    showLabel?: boolean;
    item?: Assignable;
    type?:
      | "text"
      | "number"
      | "textarea"
      | "password"
      | "email"
      | "checkbox"
      | "date"
      | "assignee"
      | "visibility";
  } = $props();

  let showPassword = $state(false);
  type Assignable = { assignee?: string };
  const baseStyle =
    "w-full rounded-2xl bg-white dark:bg-stone-950 border border-stone-100/60 dark:border-stone-800/60 px-4 py-2 text-base text-stone-900 dark:text-stone-100 placeholder-stone-400  shadow-sm focus:shadow-md focus:outline-none focus:border-accent focus:ring-2 focus:ring-accent disabled:opacity-60 disabled:cursor-not-allowed";
</script>

<div class="relative space-y-1">
  {#if showLabel}
    <Label>{labelName}</Label>
  {/if}
  {#if type === "text"}
    <input
      {name}
      type="text"
      bind:value
      bind:this={ref}
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
  {:else if type === "visibility"}
    <select {name} {value} class={baseStyle}>
      <option value="private">{m.private()}</option>
      <option value="public">{m.public()}</option>
      <option value="assigned">{m.assigned()}</option>
    </select>
  {:else if type === "assignee" && item}
    <select id="assignee" {name} class={baseStyle}>
      <option {value}>Select an assignee</option>
      {#each $studentStore as student (student.id)}
        <option
          value={student.id}
          selected={item.assignee === $user.id
            ? student.id === $assigneeStore
            : student.id === item.assignee}
        >
          {student.name}
        </option>
      {/each}
    </select>
  {/if}
</div>
