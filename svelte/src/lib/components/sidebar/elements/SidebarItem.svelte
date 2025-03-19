<script lang="ts">
  import { sidebar } from "$lib/stores/sidebar";
  import { page } from "$app/state";

  let { Icon, href = "/", name, external = false } = $props();

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
     inline-flex items-center rounded-lg p-2 font-serif transition-all
     ${
       $sidebar
         ? "hover:bg-cacao-500 justify-center  dark:hover:bg-stone-800"
         : "md:hover:translate-x-1"
     }
    ${isActive ? " bg-cacao-700  shadow-inner" : ""}
   `}
>
  {#if $sidebar}
    <div class="flex items-center justify-center">
      <Icon class="size-6 lg:size-7 xl:size-8" />
    </div>
  {:else}
    <Icon class="size-6 md:mr-2 lg:size-7 xl:size-8" />
    <p class="hidden md:block">
      {name}
    </p>
  {/if}
</a>
