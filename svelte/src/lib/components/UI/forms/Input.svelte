<script lang="ts">
  import Caption1 from "$lib/components/typography/Caption1.svelte";
  import { m } from "$lib/paraglide/messages";
  import { assigneeStore, studentStore, user } from "$lib/stores";
  import { Eye, EyeClosed } from "lucide-svelte";
  import type { ChangeEventHandler, MouseEventHandler } from "svelte/elements";

  let {
    placeholder = "Edit here",
    name,
    labelName = name.charAt(0).toUpperCase() + name.slice(1),
    value = $bindable(),
    disabled = $bindable(),
    ref,
    invalid = false,
    invalidDescription,
    required = false,
    showLabel = true,
    item,
    onchange,
    type = "text",
  }: {
    placeholder?: string;
    name: string;
    value?: string | number | boolean | null;
    labelName?: string;
    ref?: HTMLInputElement;
    invalid?: boolean;
    invalidDescription?: string;
    disabled?: boolean;
    showLabel?: boolean;
    onchange?: ChangeEventHandler<HTMLInputElement>;
    required?: boolean;
    item?: Assignable | null;
    type?:
      | "text"
      | "number"
      | "textarea"
      | "password"
      | "time"
      | "email"
      | "checkbox"
      | "date"
      | "assignee"
      | "attendee"
      | "visibility"
      | "role";
  } = $props();

  let showPassword = $state(false);
  type Assignable = { assignee?: string | null };
  const baseStyle = `w-full rounded-2xl bg-white dark:bg-stone-950  px-4 py-2 text-base placeholder-stone-400 shadow-sm focus:shadow-md focus:outline-none focus:border-accent focus:ring-2 focus:ring-accent disabled:opacity-60 disabled:cursor-not-allowed ${invalid ? "ring-error text-red-500" : "ring-default"}`;
</script>

<div class="relative space-y-1">
  {#if showLabel}
    <Caption1>{labelName}</Caption1>
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
      class="absolute {showLabel
        ? 'top-[2.65rem]'
        : 'top-[1.3rem]'} right-3 -translate-y-1/2 transform text-stone-500 dark:text-stone-300"
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
      {onchange}
    />
  {:else if type === "time"}
    <input
      type="time"
      {placeholder}
      {name}
      bind:value
      {disabled}
      class={baseStyle}
      {onchange}
    />
  {:else if type === "visibility"}
    <select name="visibility" {value} class={baseStyle}>
      <option value="private">{m.private()}</option>
      <option value="public">{m.public()}</option>
      <option value="assigned">{m.assigned()}</option>
    </select>
  {:else if type === "assignee" && item}
    <select id="assignee" name="assignee" class={baseStyle}>
      <option {value}>{placeholder}</option>
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
  {:else if type === "attendee"}
    <select id="attendee" {name} class={baseStyle}>
      <option {value}>{placeholder}</option>
      {#each $studentStore as student (student.id)}
        <option value={student.email}>
          {student.name}
        </option>
      {/each}
    </select>
  {:else if type === "role"}
    <select {name} {required} class={baseStyle}>
      <option value="">Select a role</option>
      <option value="teacher">Teacher</option>
      <option value="student">Student</option>
    </select>
  {/if}
  {#if invalid && invalidDescription}
    <Caption1 styling="text-red-500">{invalidDescription}</Caption1>
  {/if}
</div>
