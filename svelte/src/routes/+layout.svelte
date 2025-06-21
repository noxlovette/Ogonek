<script lang="ts">
  import "../app.css";
  import { Footer, Notification, MetaData } from "$lib/components";

  import { keyEscape } from "$lib/actions";

  function goBack() {
    window.history.back();
  }

  let { children } = $props();
</script>

<MetaData />

<main
  class="flex h-[100dvh] min-h-[100dvh] w-screen justify-center overflow-x-clip overflow-y-auto bg-white text-stone-800 antialiased dark:bg-stone-950 dark:text-stone-100"
  use:keyEscape={goBack}
>
  <div
    class="selection:bg-cacao-500/20 caret-cacao-800 relative flex h-full w-full flex-col items-center font-medium dark:caret-stone-200"
  >
    <div id="main" class="flex w-full flex-1 flex-col p-3 py-4">
      {@render children?.()}
    </div>
    <Footer />
  </div>

  <Notification />
</main>
<h1
  class="w-full bg-stone-100 text-center font-serif text-7xl text-stone-200 select-none xl:text-[20rem] dark:bg-stone-900 dark:text-stone-800"
>
  Ogonek
</h1>

<svelte:head>
  <title>Ogonek</title>
</svelte:head>
