<script lang="ts">
  import {
    IntroCard,
    LargeTitle,
    Merger,
    Toolbar,
    UniButton,
    VStack,
  } from "$lib/components";
  import { Flame, FlameKindling } from "lucide-svelte";
  import { m } from "$lib/paraglide/messages";
  import WorkArea from "$lib/components/UI/WorkArea.svelte";
  import HStack from "$lib/components/UI/HStack.svelte";
  import Divider from "$lib/components/UI/toolbar/Divider.svelte";
</script>

<WorkArea>
  <Toolbar>
    <LargeTitle>
      {m.welcomeTo()}<span class="font-serif italic">Ogonek</span>
    </LargeTitle>
    <Divider />
    <Merger>
      <UniButton iconOnly={false} Icon={Flame} href="/auth/login"
        >{m.logIn()}</UniButton
      >
      <UniButton
        iconOnly={false}
        Icon={FlameKindling}
        href="/auth/signup"
        variant="primary"
      >
        {m.signUp()}</UniButton
      >
    </Merger>
  </Toolbar>
  <HStack>
    <IntroCard title={m.lessons()}>{m.mellow_ok_blackbird_peek()}</IntroCard>
    <IntroCard title={m.livid_small_gadfly_gulp()}
      >{m.small_deft_goose_grin()}</IntroCard
    >
    <IntroCard title={m.flashcards()}>{m.calm_blue_warbler_zoom()}</IntroCard>
  </HStack>
</WorkArea>
