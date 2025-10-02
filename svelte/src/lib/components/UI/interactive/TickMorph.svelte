<script lang="ts">
  import { Check } from "@lucide/svelte";
  import { VStack } from "../layout";
  import Caption1 from "$lib/components/typography/Caption1.svelte";

  let {
    value = $bindable(false),
    name = "morph-checkbox",
    title = "Morph pop",
    hint = "Full checkmark with pop effect.",
    disabled = false,
    size = 24,
  }: {
    value?: boolean;
    name?: string;
    title?: string;
    hint?: string;
    disabled?: boolean;
    size?: number;
  } = $props();

  const tileSize = size;
  const containerSize = size * 1.4;
  const tickSize = Math.max(16, size * 0.67);

  const uniqueId = `checkbox-${name}-${Math.random().toString(36).substr(2, 9)}`;

  function handleKeydown(event: KeyboardEvent) {
    if (!disabled && (event.key === " " || event.key === "Enter")) {
      event.preventDefault();
      value = !value;
    }
  }
</script>

<VStack styling="items-center" override="gap-2">
  <button
    type="button"
    class="group inline-grid cursor-pointer place-items-center {disabled
      ? 'cursor-not-allowed opacity-50'
      : ''}"
    style="width: {containerSize}px; height: {containerSize}px;"
    onkeydown={handleKeydown}
  >
    <input
      {name}
      type="checkbox"
      bind:checked={value}
      {disabled}
      tabindex="-1"
      class="sr-only"
      id={uniqueId}
    />
    <label
      for={uniqueId}
      class="grid cursor-pointer place-items-center rounded-lg border-2 transition-all duration-200 ease-out active:scale-95
               {value ? 'border-accent bg-accent/15 ' : 'border-hover'}"
      style="width: {tileSize}px; height: {tileSize}px;"
    >
      <Check
        class="text-accent size-4 transition-all duration-150 ease-out {value
          ? 'scale-100 opacity-100'
          : 'scale-80 opacity-0'}"
        width={tickSize}
        height={tickSize}
        strokeWidth={3}
      />
    </label>
  </button>
  <Caption1>
    {title}
  </Caption1>
</VStack>
