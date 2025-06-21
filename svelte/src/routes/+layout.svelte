<script lang="ts">
  import "../app.css";
  import { Header, Footer, Notification, MetaData } from "$lib/components";

  import { keyEscape } from "$lib/actions";

  function goBack() {
    window.history.back();
  }

  let { children } = $props();
</script>

<MetaData />

<main
  class="flex h-screen w-screen justify-center overflow-x-clip overflow-y-auto bg-white text-stone-800 antialiased dark:bg-stone-950 dark:text-stone-100"
  use:keyEscape={goBack}
>
  <div
    class="selection:bg-cacao-500/20 caret-cacao-800 relative flex h-full w-full flex-col items-center font-medium dark:caret-stone-200"
  >
    <div id="main" class="flex w-full flex-1 flex-col px-3 md:px-4">
      {@render children?.()}
    </div>
    <Footer />
  </div>
  <Notification />
</main>

<svelte:head>
  <title>Ogonek</title>
</svelte:head>
