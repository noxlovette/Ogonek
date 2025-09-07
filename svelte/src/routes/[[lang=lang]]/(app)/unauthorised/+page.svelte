<script lang="ts">
  import { Lock, ArrowLeft } from "lucide-svelte";
  import { m } from "$lib/paraglide/messages";
  import {
    Callout,
    HStack,
    LargeTitle,
    Merger,
    UniButton,
  } from "$lib/components";
</script>

<div class="flex size-full flex-col items-center justify-center">
  <HStack>
    <LargeTitle>
      {m.noisy_such_bear_achieve()}
    </LargeTitle>

    <Callout>
      {m.stale_vivid_dolphin_thrive()}
    </Callout>

    <Merger>
      <UniButton
        Icon={ArrowLeft}
        iconOnly={false}
        onclick={() => window.history.back()}
        >{m.level_low_angelfish_peel()}</UniButton
      >
    </Merger>
  </HStack>
</div>
<svelte:head>
  <title>{m.brave_fair_cobra_tear()}</title>
</svelte:head>
