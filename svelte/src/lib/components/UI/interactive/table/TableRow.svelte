<script lang="ts">
  import { goto } from "$app/navigation";
  import type { Snippet } from "svelte";

  interface Props {
    href: string;
    children: Snippet;
  }

  const { href, children }: Props = $props();

  function handleRowClick(event: KeyboardEvent | MouseEvent) {
    if (
      event instanceof KeyboardEvent &&
      event.key !== "Enter" &&
      event.key !== " "
    ) {
      return;
    }
    goto(href);
  }
</script>

<button
  class="flex w-full items-center justify-between gap-2 px-2 py-2.5"
  type="button"
  onclick={(e) => handleRowClick(e)}
  onkeydown={(e) => handleRowClick(e)}
  tabindex="0"
  data-cy="table-row-clickable"
>
  {@render children()}
</button>
