<script lang="ts">
  import { Header, IntroCard, UniButton } from "$lib/components";
  import { Flame, FlameKindling } from "lucide-svelte";
  import { m } from "$lib/paraglide/messages";
</script>

<Header />
<div class="flex w-full flex-col items-center space-y-4 md:p-6">
  <h1 class="text-3xl font-bold tracking-tighter md:text-4xl lg:text-6xl">
    {m.welcomeTo()}<span class="font-serif italic">Ogonek</span>
  </h1>
  <grid class="grid gap-3 md:grid-cols-3">
    <IntroCard title="Lessons">{m.mellow_ok_blackbird_peek()}</IntroCard>
    <IntroCard title="Homework">{m.small_deft_goose_grin()}</IntroCard>
    <IntroCard title="Flashcards">{m.calm_blue_warbler_zoom()}</IntroCard>
  </grid>
  <div class="grid grid-cols-2 gap-3">
    <UniButton
      Icon={Flame}
      iconPosition="right"
      href="/auth/login"
      variant="secondary">{m.logIn()}</UniButton
    >
    <UniButton Icon={FlameKindling} href="/auth/signup" variant="primary">
      {m.signUp()}</UniButton
    >
  </div>
</div>
