<script lang="ts">
  import { sidebar } from "$lib/stores/sidebar";

  let { Icon, href = "/", name } = $props();
</script>

<a
  {href}
  class={`
     inline-flex items-center font-serif transition-all
     ${
       $sidebar
         ? "hover:bg-cacao-500 dark:hover:bg-milk-800 justify-center rounded-full p-2"
         : "md:px-1 md:py-2 md:hover:translate-x-1 lg:px-2 lg:py-4"
     }
   `}
>
  {#if $sidebar}
    <div class="flex items-center justify-center">
      <Icon class="size-6 lg:size-7 xl:size-8" />
    </div>
  {:else}
    <Icon class="size-6 md:mr-2 lg:size-7 xl:size-8" />
    <p>
      {name}
    </p>
  {/if}
</a>
