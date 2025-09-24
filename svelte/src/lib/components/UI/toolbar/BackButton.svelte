<script lang="ts">
  import { page } from "$app/state";
  import { ChevronLeft } from "lucide-svelte";
  import { Merger } from ".";
  import UniButton from "../forms/buttons/UniButton.svelte";
  import VStack from "../VStack.svelte";
  import Divider from "./Divider.svelte";
  import { m } from "$lib/paraglide/messages";

  const { override = false } = $props();

  const showBack = $derived.by(() => {
    const segments = page.url.pathname.split("/").filter(Boolean);

    const [firstSegment] = segments;

    if (!isAppRoot(firstSegment)) {
      return segments.length > 1;
    }

    if (segments.length < 2) return false;

    const [role, section, maybeId] = segments;

    if (section === "dashboard") return false;

    return segments.length > 2;
  });

  const href = $derived(
    page.url.pathname.split("/").slice(0, -1).join("/") || "/",
  );

  function isAppRoot(segment: string): segment is "t" | "s" | "admin" {
    return segment === "t" || segment === "s" || segment === "admin";
  }
</script>

{#if showBack && !override}
  <VStack>
    <Merger>
      <UniButton
        content={m.new_nice_parakeet_arise()}
        ariaLabel="Back"
        {href}
        Icon={ChevronLeft}
      ></UniButton>
    </Merger>
    <Divider></Divider>
  </VStack>
{/if}
