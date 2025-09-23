<script lang="ts">
  import Caption1 from "$lib/components/typography/Caption1.svelte";
  import { m } from "$lib/paraglide/messages";
  import { assigneeStore, studentStore, user } from "$lib/stores";
  import { Eye, EyeClosed } from "lucide-svelte";
  import type { ChangeEventHandler } from "svelte/elements";

  let {
    placeholder = "Менять тут",
    name,
    labelName = name.charAt(0).toUpperCase() + name.slice(1),
    value = $bindable(),
    disabled = $bindable(),
    ref = $bindable(),
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
      | "scope"
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
      {required}
      aria-label={showLabel ? undefined : labelName}
      aria-invalid={invalid}
      aria-describedby={invalid && invalidDescription
        ? `${name}-error`
        : undefined}
    />
  {:else if type === "textarea"}
    <textarea
      {name}
      rows="3"
      bind:value
      {disabled}
      class={baseStyle + " resize-none"}
      {placeholder}
      {required}
      aria-label={showLabel ? undefined : labelName}
      aria-invalid={invalid}
      aria-describedby={invalid && invalidDescription
        ? `${name}-error`
        : undefined}
    ></textarea>
  {:else if type === "number"}
    <input
      type="number"
      {placeholder}
      {name}
      {disabled}
      bind:value
      class={baseStyle}
      {required}
      aria-label={showLabel ? undefined : labelName}
      aria-invalid={invalid}
      aria-describedby={invalid && invalidDescription
        ? `${name}-error`
        : undefined}
    />
  {:else if type === "password"}
    <input
      type={showPassword ? "text" : "password"}
      {placeholder}
      {name}
      bind:value
      {disabled}
      class={baseStyle}
      {required}
      aria-label={showLabel ? undefined : labelName}
      aria-invalid={invalid}
      aria-describedby={invalid && invalidDescription
        ? `${name}-error`
        : undefined}
    />
    <button
      type="button"
      class="absolute {showLabel
        ? 'top-[2.65rem]'
        : 'top-[1.3rem]'} right-3 -translate-y-1/2 transform text-stone-500 dark:text-stone-300"
      onclick={() => (showPassword = !showPassword)}
      aria-label={showPassword ? "Hide password" : "Show password"}
      tabindex="0"
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
      {required}
      aria-label={showLabel ? undefined : labelName}
      aria-invalid={invalid}
      aria-describedby={invalid && invalidDescription
        ? `${name}-error`
        : undefined}
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
      {required}
      aria-label={showLabel ? undefined : labelName}
      aria-invalid={invalid}
      aria-describedby={invalid && invalidDescription
        ? `${name}-error`
        : undefined}
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
      {required}
      aria-label={showLabel ? undefined : labelName}
      aria-invalid={invalid}
      aria-describedby={invalid && invalidDescription
        ? `${name}-error`
        : undefined}
    />
  {:else if type === "visibility"}
    <select
      name="visibility"
      {value}
      class={baseStyle}
      {required}
      aria-label={showLabel ? undefined : labelName}
      aria-invalid={invalid}
      aria-describedby={invalid && invalidDescription
        ? `${name}-error`
        : undefined}
    >
      <option value="private">{m.private()}</option>
      <option value="public">{m.public()}</option>
      <option value="assigned">{m.assigned()}</option>
    </select>
  {:else if type === "assignee" && item}
    <select
      id="assignee"
      name="assignee"
      class={baseStyle}
      {required}
      aria-label={showLabel ? undefined : labelName}
      aria-invalid={invalid}
      aria-describedby={invalid && invalidDescription
        ? `${name}-error`
        : undefined}
    >
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
    <select
      id="attendee"
      {name}
      class={baseStyle}
      {required}
      aria-label={showLabel ? undefined : labelName}
      aria-invalid={invalid}
      aria-describedby={invalid && invalidDescription
        ? `${name}-error`
        : undefined}
    >
      <option {value}>{placeholder}</option>
      {#each $studentStore as student (student.id)}
        <option value={student.id} selected={student.id === value}>
          {student.name}
        </option>
      {/each}
    </select>
  {:else if type === "role"}
    <select
      {name}
      {required}
      class={baseStyle}
      aria-label={showLabel ? undefined : labelName}
      aria-invalid={invalid}
      aria-describedby={invalid && invalidDescription
        ? `${name}-error`
        : undefined}
    >
      <option value="">{m.cool_seemly_raven_walk()}</option>
      <option value="teacher">Teacher</option>
      <option value="student">Student</option>
    </select>
  {:else if type === "scope"}
    <select
      {name}
      {required}
      class={baseStyle}
      aria-label={showLabel ? undefined : labelName}
      aria-invalid={invalid}
      aria-describedby={invalid && invalidDescription
        ? `${name}-error`
        : undefined}
    >
      <option value="this-only">Только это событие</option>
      <option value="this-and-future">Это и следующие события</option>
    </select>
  {/if}
  {#if invalid && invalidDescription}
    <Caption1 id="{name}-error">
      {invalidDescription}
    </Caption1>
  {/if}
</div>
