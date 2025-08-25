<script lang="ts">
  import { page } from "$app/state";
  import { ChevronLeft } from "lucide-svelte";
  import { derived } from "svelte/store";
  import UniButton from "../forms/UniButton.svelte";
  import Merger from "./Merger.svelte";
  import Divider from "./Divider.svelte";
  import { VStack } from "..";

  let { children } = $props();

  // Determine if we’re in a "subsection"
  // Example: hide on `/role/dashboard`, show on `/role/lessons/:id`
  const showBack = $derived.by(() => {
    const segments = page.url.pathname.split("/").filter(Boolean);

    if (segments.length < 2) return false;

    const [role, section, maybeId] = segments;

    if (section === "dashboard") return false;

    return segments.length > 2;
  });

  function goBack() {
    window.history.back();
  }
</script>

<div class="flex w-full flex-col justify-between gap-4">
  {#if showBack}
    <VStack>
      <Merger>
        <UniButton onclick={goBack} Icon={ChevronLeft}></UniButton>
      </Merger>
      <Divider></Divider>
    </VStack>
  {/if}
  <div class="flex">
    {@render children?.()}
  </div>
</div>
