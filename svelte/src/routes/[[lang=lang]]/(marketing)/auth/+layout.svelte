<script lang="ts">
  import { Grid, HStack, LargeTitle } from "$lib/components";
  import { m } from "$lib/paraglide/messages";

  import { page } from "$app/state";

  const isLogin = $derived(page.url.href.includes("login"));

  let { children } = $props();
</script>

<Grid>
  <HStack>
    <LargeTitle
      >{isLogin
        ? m.zany_few_goose_mop()
        : m.bad_even_seahorse_rise()}</LargeTitle
    >
    <p class="mt-2 text-sm text-stone-600">
      {isLogin ? m.petty_neat_emu_endure() : m.dark_candid_octopus_compose()}
      <a
        href={isLogin ? "/auth/signup" : "/auth/login"}
        class="text-accent hover:text-accent font-medium dark:text-stone-50 dark:hover:text-stone-200"
        >{isLogin ? m.logIn() : m.signUp()}</a
      >
    </p>
    <img
      src="/images/logo.png"
      class="hidden size-48 md:block"
      alt="Ogonek Logo"
    />
  </HStack>

  {@render children?.()}
</Grid>
