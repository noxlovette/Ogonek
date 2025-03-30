<script lang="ts">
  import { mobileMenuOpen } from "$lib/stores";

  interface MobileMenuProps {
    elements: any[];
  }

  let { elements = [] }: MobileMenuProps = $props();
</script>

{#if $mobileMenuOpen}
  <div class="fixed inset-0 z-50 flex flex-col">
    <button
      class="absolute inset-0 bg-stone-900/60 backdrop-blur-sm"
      onclick={() => mobileMenuOpen.false()}
      aria-label="Close menu"
    ></button>

    <!-- Menu panel -->
    <div
      class="bg-cacao-600 text-cacao-50 fixed right-0 bottom-0 left-0 z-10 rounded-t-2xl shadow-xl ring-1 ring-stone-200 dark:bg-stone-900 dark:text-stone-100 dark:ring-stone-700"
    >
      <div
        class="mx-auto mt-3 h-1 w-16 rounded-full bg-stone-300 dark:bg-stone-600"
      ></div>

      <nav class="p-6">
        <ul class="flex w-full flex-col items-end space-y-4">
          {#each elements as Element}
            <li
              class="transition-all duration-300 ease-out"
              onclick={() => mobileMenuOpen.false()}
            >
              <Element />
            </li>
          {/each}
        </ul>
      </nav>
    </div>
  </div>
{/if}
