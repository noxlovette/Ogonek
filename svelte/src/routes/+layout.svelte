<script lang="ts">
  import "../app.css";
  import { Header, Footer, Notification, MetaData } from "$lib/components";
  import SearchOverlay from "$lib/components/UI/search/SearchOverlay.svelte";
  import { keyEscape } from "$lib/actions";

  function goBack() {
    window.history.back();
  }

  interface Props {
    children?: import("svelte").Snippet;
  }
  let { children }: Props = $props();
</script>

<MetaData />

<main
  class="bg-milk-50 dark:bg-milk-950 text-cacao-800 dark:text-milk-100 h-screen w-screen overflow-auto antialiased"
  use:keyEscape={goBack}
>
  <div
    class="selection:bg-cacao-500/20 dark:caret-milk-200 caret-cacao-800 mx-auto flex h-full max-w-7xl flex-col items-center font-medium"
  >
    <Header />
    <div id="main" class="flex w-full flex-1 flex-row py-4">
      {@render children?.()}
    </div>
    <Footer />
  </div>
  <Notification />
  <SearchOverlay />
</main>

<svelte:head>
  <title>Ogonek</title>
</svelte:head>
