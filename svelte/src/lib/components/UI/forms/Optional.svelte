<script lang="ts">
  import { Callout } from "$lib/components/typography";
  import type { Snippet } from "svelte";
  import { Plus } from "@lucide/svelte";

  let {
    children,
    toggle = $bindable(false),
  }: { children: Snippet; toggle?: boolean } = $props();
</script>

<button
  type="button"
  onclick={() => (toggle = true)}
  class=" bg-default ring-default group flex justify-between rounded-xl p-2"
>
  <Callout styling="opacity-40 group-hover:opacity-80">
    {@render children()}
  </Callout>
  <Plus class="opacity-40 group-hover:opacity-80"></Plus>
</button>
