<script lang="ts">
  import "../app.css";
  import { env } from "$env/dynamic/public";
  import {
    Footer,
    Notification,
    MetaData,
    MockController,
  } from "$lib/components";

  import { keyEscape } from "$lib/actions";
  import { page } from "$app/state";

  function goBack() {
    window.history.back();
  }

  let { children } = $props();

  const isApp = $derived(
    page.url.pathname.includes("/s/") || page.url.pathname.includes("/t/"),
  );

  let width = $state(0);
  const isMobile = $derived(width < 768);
  const showExtended = $derived(!(isMobile && isApp));
</script>

<svelte:window bind:innerWidth={width} />

<MetaData />

<main
  class="flex min-h-[100dvh] w-full flex-col justify-center overflow-clip bg-white text-stone-800 antialiased select-none dark:bg-stone-950 dark:text-stone-100"
  use:keyEscape={goBack}
>
  <Notification />
  <div
    class="caret-accent relative flex h-full min-h-[100dvh] w-full flex-col items-center font-medium shadow-xl selection:bg-stone-200 dark:caret-stone-200 dark:shadow-stone-600/10 dark:selection:bg-stone-700"
  >
    <div
      id="main"
      class="mb-20 flex w-full max-w-7xl flex-1 flex-col p-3.5 sm:p-6 md:mb-0 md:p-6 lg:p-10"
    >
      {@render children?.()}
    </div>
    {#if showExtended}
      <Footer />
    {/if}
  </div>
  {#if env.PUBLIC_MOCK_MODE}
    <MockController></MockController>
  {/if}

  {#if showExtended}
    <h1
      class="w-full cursor-default bg-stone-100 text-center font-serif text-7xl tracking-tighter text-stone-200 select-none xl:text-[20rem] dark:bg-stone-900 dark:text-stone-800"
    >
      Ogonëk
    </h1>
  {/if}
</main>

<svelte:head>
  <title>Ogonek</title>

  {#if env.PUBLIC_GOOGLE_SITE_VERIFICATION}
    <meta
      name="google-site-verification"
      content={env.PUBLIC_GOOGLE_SITE_VERIFICATION}
    />
  {/if}
  {#if env.PUBLIC_YANDEX_VERIFICATION}
    <meta name="yandex-verification" content={env.PUBLIC_YANDEX_VERIFICATION} />
  {/if}

  <meta name="robots" content="noindex, nofollow" />
</svelte:head>
