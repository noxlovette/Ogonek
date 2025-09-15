<script lang="ts">
  import { page } from "$app/state";
  import { ChevronLeft } from "lucide-svelte";
  import UniButton from "../forms/buttons/UniButton.svelte";
  import Merger from "./Merger.svelte";
  import Divider from "./Divider.svelte";
  import { VStack } from "..";
  import type { Role } from "$lib/types";

  let { children, src = "", alt = "" } = $props();
  function isAppRoot(segment: string): segment is "t" | "s" | "admin" {
    return segment === "t" || segment === "s" || segment === "admin";
  }

  const showBack = $derived.by(() => {
    const segments = page.url.pathname.split("/").filter(Boolean);

    const [firstSegment] = segments;

    if (!isAppRoot(firstSegment)) {
      return segments.length > 1;
    }

    if (segments.length < 2) return false;

    const [role, section, maybeId] = segments;

    if (section === "dashboard" || section === "calendar") return false;

    return segments.length > 2;
  });

  function goBack() {
    window.history.back();
  }
</script>

<div
  class="relative flex w-full flex-col justify-between gap-4 py-2 md:py-3 lg:py-4"
>
  {#if src}
    <div class="absolute inset-0 opacity-50">
      <img class="size-full rounded-xl object-cover" {src} {alt} />
    </div>
  {/if}
  {#if showBack}
    <VStack>
      <Merger>
        <UniButton onclick={goBack} Icon={ChevronLeft}></UniButton>
      </Merger>
      <Divider></Divider>
    </VStack>
  {/if}

  <div class="relative z-10 flex">
    {@render children?.()}
  </div>
</div>
