<script lang="ts">
  import { rightbar } from "$lib/stores/sidebar";

  let { elements }: { elements: ConstructorOfATypedSvelteComponent[] } =
    $props();
</script>

<div
  class={`
relative
     -mr-8 h-max flex-shrink-0 flex-col overflow-hidden py-2 pb-5
     transition-all duration-300 ease-in-out
     ${$rightbar ? "w-0" : "w-1/5"}
     order-last hidden md:flex md:text-lg lg:text-xl
     xl:text-2xl
   `}
>
  <div class="w-full">
    <ul
      class={`
         flex flex-col
         ${$rightbar ? "items-center space-y-6 pt-4" : " space-y-2 px-4 lg:space-y-3 xl:space-y-4"}
       `}
    >
      {#each elements as Element}
        <Element />
      {/each}
    </ul>
  </div>
</div>
