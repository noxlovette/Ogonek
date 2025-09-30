<script lang="ts">
  import { Caption1, Grid, HStack, LargeTitle, Toolbar } from "$lib/components";
  import { m } from "$lib/paraglide/messages";

  import { page } from "$app/state";
  import WorkArea from "$lib/components/UI/layout/WorkArea.svelte";

  const isLogin = $derived(
    page.url.href.includes("login") || page.url.href.includes("bind"),
  );

  let { children } = $props();
</script>

<WorkArea>
  <Toolbar>
    <HStack>
      <LargeTitle
        >{isLogin
          ? m.zany_few_goose_mop()
          : m.bad_even_seahorse_rise()}</LargeTitle
      >
      <Caption1>
        {isLogin ? m.petty_neat_emu_endure() : m.dark_candid_octopus_compose()}
        <a
          data-cy={isLogin ? "signup-button" : "login-button"}
          href={isLogin ? "/signup" : "/login"}
          class="text-accent"
          >{isLogin ? m.nimble_north_worm_drop() : m.logIn()}</a
        >
      </Caption1>
    </HStack>
  </Toolbar>

  <Grid>
    {@render children?.()}
  </Grid>
</WorkArea>
