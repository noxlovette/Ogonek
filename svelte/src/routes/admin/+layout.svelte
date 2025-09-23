<script lang="ts">
  import { Body, Content, Dashboard, Sidebar, WorkArea } from "$lib/components";
  import type { LayoutProps } from "./$types";
  import Loader from "$lib/components/UI/navigation/Loader.svelte";
  import { setUser } from "$lib/stores";

  let { data, children }: LayoutProps = $props();

  setUser(data.context.user);
</script>

<div class="md:hidden">
  <Body>Not available on mobile</Body>
</div>
<div class="hidden flex-row gap-4 p-2 md:flex md:gap-6 md:p-5 lg:gap-8 lg:p-6">
  <div class="hidden w-max flex-col md:block">
    <Sidebar><Dashboard /><Content /></Sidebar>
  </div>
  <WorkArea>
    {@render children?.()}
  </WorkArea>

  <Loader />
</div>

<svelte:head>
  <title>Ogonek | Admin</title>
</svelte:head>
