<script lang="ts">
  import { Check } from "@lucide/svelte";
  import { VStack } from "../layout";
  import Caption1 from "$lib/components/typography/Caption1.svelte";

  let {
    value = $bindable(),
    name = "morph-checkbox",
    group = $bindable([]),
    title = "Morph pop",
    noText = false,
    disabled = false,
    size = 24,
  }: {
    value: boolean | string;
    name?: string;
    title?: string;
    group?: (string | number)[];
    hint?: string;
    noText?: boolean;
    disabled?: boolean;
    size?: number;
  } = $props();

  const tileSize = size;
  const containerSize = size * 1.4;
  const tickSize = Math.max(16, size * 0.67);

  const isChecked = $derived(
    group && typeof value === "string" ? group.includes(value) : !!value,
  );

  function toggle() {
    if (disabled) return;

    if (group && typeof value === "string") {
      if (group.includes(value)) {
        group = group.filter((item) => item !== value);
      } else {
        group = [...group, value];
      }
    } else {
      value = !value;
    }
  }
</script>

<VStack styling="items-center" override="gap-2">
  <button
    type="button"
    onclick={toggle}
    class="group inline-grid cursor-pointer place-items-center {disabled
      ? 'cursor-not-allowed opacity-50'
      : ''}"
    style="width: {containerSize}px; height: {containerSize}px;"
  >
    <div
      class="grid cursor-pointer place-items-center rounded-lg border-2 transition-all duration-200 ease-out active:scale-95
               {isChecked ? 'border-accent bg-accent/15 ' : 'border-hover'}"
      style="width: {tileSize}px; height: {tileSize}px;"
    >
      <Check
        class="text-accent size-4 transition-all duration-150 ease-out {isChecked
          ? 'scale-100 opacity-100'
          : 'scale-80 opacity-0'}"
        width={tickSize}
        height={tickSize}
        strokeWidth={3}
      />
    </div>
  </button>
  {#if !noText}
    <Caption1>
      {title}
    </Caption1>
  {/if}
</VStack>
