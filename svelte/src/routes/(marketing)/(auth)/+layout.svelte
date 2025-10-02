<script lang="ts">
  import {
    Caption1,
    Grid,
    Header,
    HStack,
    LargeTitle,
    Title1,
    Title3,
    Toolbar,
  } from "$lib/components";
  import { m } from "$lib/paraglide/messages";

  import { page } from "$app/state";

  const isLogin = $derived(
    page.url.href.includes("login") || page.url.href.includes("bind"),
  );

  let { children } = $props();
</script>

<div class="gap-default mx-auto flex flex-col items-center justify-center">
  <LargeTitle styling="text-center"
    >{isLogin ? m.zany_few_goose_mop() : m.bad_even_seahorse_rise()}</LargeTitle
  >
  <Caption1>
    {isLogin ? m.petty_neat_emu_endure() : m.dark_candid_octopus_compose()}
    <a
      data-cy={isLogin ? "signup-button" : "login-button"}
      href={isLogin ? "/signup" : "/login"}
      class="text-accent">{isLogin ? m.nimble_north_worm_drop() : m.logIn()}</a
    >
  </Caption1>

  {@render children?.()}
</div>
