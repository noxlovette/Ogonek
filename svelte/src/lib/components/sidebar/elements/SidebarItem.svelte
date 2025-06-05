<script lang="ts">
  import { sidebar } from "$lib/stores/sidebar";
  import { page } from "$app/state";

  import H3 from "$lib/components/typography/H3.svelte";
  import type { Component } from "svelte";

  let {
    Icon,
    href = "/",
    name,
    external = false,
    rightBar = false,
  }: {
    Icon?: Component;
    href: string;
    name: string;
    external?: boolean;
    rightBar: boolean;
  } = $props();

  const target = $derived(external === true ? "_blank" : undefined);
  const rel = $derived(external === true ? "noopener noreferrer" : undefined);

  const isActive = $derived(
    page.url.pathname.toLowerCase().includes(name.toLowerCase()),
  );
</script>

<a
  {href}
  {target}
  {rel}
  class={`
     inline-flex items-center rounded-lg px-2 py-1 transition-transform 
     ${
       $sidebar
         ? "hover:bg-cacao-400 hover:text-cacao-50 justify-center dark:hover:bg-stone-800"
         : "md:hover:translate-x-1"
     }
    ${isActive ? " bg-cacao-500 gradient-to-br from-cacao-50 text-cacao-50 dark:bg-cacao-600 dark:text-cacao-50 ring-cacao-700/40 shadow-inner ring-1" : ""}
   `}
>
  {#if $sidebar && !rightBar}
    <div class="flex items-center justify-center">
      <Icon class="size-6 lg:size-7 " />
    </div>
  {:else}
    <Icon class="mr-2 size-6 lg:size-7 " />

    <H3>
      {name}
    </H3>
  {/if}
</a>
