<script lang="ts">
  import Caption1 from "$lib/components/typography/Caption1.svelte";
  import { assigneeStore, studentStore, user } from "$lib/stores";
  import {
    BadgeCheck,
    BadgeQuestionMark,
    Eye,
    EyeClosed,
  } from "@lucide/svelte";
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
    dataCy,
    verified = false,
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
    dataCy?: string;
    verified?: boolean;
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
  const baseStyle = `input-default ${invalid ? "ring-error text-red-500" : "ring-default"}`;
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
      data-cy={dataCy}
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
      data-cy={dataCy}
      class={baseStyle + "resize-none"}
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
      data-cy={dataCy}
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
      data-cy={dataCy}
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
      data-cy={dataCy}
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
      data-cy={dataCy}
      class={baseStyle}
      {required}
      aria-label={showLabel ? undefined : labelName}
      aria-invalid={invalid}
      aria-describedby={invalid && invalidDescription
        ? `${name}-error`
        : undefined}
    />
    {#if verified}
      <BadgeCheck class="absolute top-8 right-2 text-emerald-500" />
    {:else}
      <BadgeQuestionMark class="absolute top-8 right-2 text-red-500" />
    {/if}
  {:else if type === "date"}
    <input
      type="date"
      data-cy={dataCy}
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
      data-cy={dataCy}
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
      bind:value
      class={baseStyle}
      {required}
      data-cy={dataCy}
      aria-label={showLabel ? undefined : labelName}
      aria-invalid={invalid}
      aria-describedby={invalid && invalidDescription
        ? `${name}-error`
        : undefined}
    >
      <option value="private">Только я</option>
      <option value="shared">Кто-то</option>
      <option value="public">Кто угодно</option>
    </select>
  {:else if type === "assignee" && item}
    <select
      id="assignee"
      name="assignee"
      data-cy={dataCy}
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
      data-cy={dataCy}
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
      data-cy={dataCy}
      class={baseStyle}
      aria-label={showLabel ? undefined : labelName}
      aria-invalid={invalid}
      aria-describedby={invalid && invalidDescription
        ? `${name}-error`
        : undefined}
    >
      <option value="">Выберите роль</option>
      <option value="teacher">Учитель</option>
      <option value="student">Ученик</option>
    </select>
  {:else if type === "scope"}
    <select
      {name}
      {required}
      class={baseStyle}
      data-cy={dataCy}
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
    <Caption1 override="text-red-600" id="{name}-error">
      {invalidDescription}
    </Caption1>
  {/if}
</div>
