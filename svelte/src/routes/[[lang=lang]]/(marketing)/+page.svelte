<script lang="ts">
  import { IntroCard, UniButton } from "$lib/components";
  import { Flame, FlameKindling } from "lucide-svelte";
  import { m } from "$lib/paraglide/messages";
</script>

<div class="flex w-full flex-col items-center space-y-4 md:p-6">
  <h1 class="text-3xl font-bold tracking-tighter md:text-4xl lg:text-6xl">
    {m.welcomeTo()}<span class="font-serif italic">Ogonek</span>
  </h1>
  <grid class="grid gap-3 md:grid-cols-3">
    <IntroCard title={m.lessons()}>{m.mellow_ok_blackbird_peek()}</IntroCard>
    <IntroCard title={m.livid_small_gadfly_gulp()}
      >{m.small_deft_goose_grin()}</IntroCard
    >
    <IntroCard title={m.flashcards()}>{m.calm_blue_warbler_zoom()}</IntroCard>
  </grid>
  <div class="grid grid-cols-2 gap-3">
    <UniButton Icon={Flame} href="/auth/login" variant="secondary"
      >{m.logIn()}</UniButton
    >
    <UniButton Icon={FlameKindling} href="/auth/signup" variant="primary">
      {m.signUp()}</UniButton
    >
  </div>
</div>
