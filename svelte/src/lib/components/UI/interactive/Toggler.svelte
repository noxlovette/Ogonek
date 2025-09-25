<script lang="ts">
  import Caption1 from "$lib/components/typography/Caption1.svelte";
  import { VStack } from "../layout";

  let {
    value = $bindable(false),
    name = "toggle",
    title = "Toggle",
    disabled = false,
  }: {
    value?: boolean;
    name?: string;
    title?: string;
    disabled?: boolean;
  } = $props();

  // Generate unique ID
  const uniqueId = `toggle-${name}-${Math.random().toString(36).substr(2, 9)}`;

  // Handler pour éviter les conflits de binding
  function handleClick() {
    if (!disabled) {
      value = !value;
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (!disabled && (event.key === " " || event.key === "Enter")) {
      event.preventDefault();
      value = !value;
    }
  }
</script>

<VStack styling="items-center group" override="gap-2">
  <button
    type="button"
    role="switch"
    aria-checked={value}
    aria-labelledby={`${uniqueId}-label`}
    class=" relative h-8 w-14 overflow-hidden rounded-full border-2 transition-all duration-150 ease-out focus:outline-none
           {disabled ? 'cursor-not-allowed opacity-50' : ''}
           {value ? ' bg-accent/12 border-accent ' : 'border-hover'}"
    {disabled}
    onclick={handleClick}
    onkeydown={handleKeydown}
  >
    <!-- Hidden input pour les forms -->
    <input
      id={uniqueId}
      {name}
      type="checkbox"
      checked={value}
      {disabled}
      class="sr-only"
      tabindex="-1"
      aria-hidden="true"
      readonly
    />

    <!-- Knob avec de meilleures animations -->
    <span
      class="absolute top-[2.2px] size-6 rounded-full bg-stone-50 transition-all duration-300 ease-out
             {value ? 'left-[26px]' : 'left-[3px]'}"
      style="transition-timing-function: cubic-bezier(0.2, 0.9, 0.3, 1);"
    >
    </span>
  </button>

  <Caption1 id="{uniqueId}-label">
    {title}
  </Caption1>
</VStack>
