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

  function goBack() {
    window.history.back();
  }

  let { children } = $props();
</script>

<MetaData />

<main
  class="flex min-h-[100dvh] w-full flex-col justify-center overflow-x-clip overflow-y-auto bg-white text-stone-800 antialiased select-none dark:bg-stone-950 dark:text-stone-100"
  use:keyEscape={goBack}
>
  <div
    class="selection:bg-accent caret-accent relative flex h-full min-h-[100dvh] w-full flex-col items-center font-medium dark:caret-stone-200"
  >
    <div id="main" class="flex w-full max-w-7xl flex-1 flex-col p-3 py-4">
      {@render children?.()}
    </div>
    <Footer />
    <Notification />
  </div>
  {#if env.PUBLIC_MOCK_MODE}
    <MockController></MockController>
  {/if}
  <h1
    class="w-full cursor-default text-center font-serif text-7xl text-stone-200 xl:text-[20rem] dark:text-stone-800"
  >
    Ogonek
  </h1>
</main>

<svelte:head>
  <title>Ogonek</title>
</svelte:head>
