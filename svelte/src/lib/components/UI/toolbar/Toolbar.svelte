<script lang="ts">
  import { page } from "$app/state";
  import { ChevronLeft } from "lucide-svelte";
  import UniButton from "../forms/buttons/UniButton.svelte";
  import Merger from "./Merger.svelte";
  import Divider from "./Divider.svelte";
  import { VStack } from "..";
  import type { Role } from "$lib/types";

  let { children } = $props();
  function isRole(segment: string): segment is Role {
    return segment === "t" || segment === "s";
  }

  const showBack = $derived.by(() => {
    const segments = page.url.pathname.split("/").filter(Boolean);

    const [firstSegment] = segments;

    if (!isRole(firstSegment)) {
      return segments.length > 1;
    }

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
