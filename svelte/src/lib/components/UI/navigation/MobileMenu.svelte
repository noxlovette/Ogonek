<script lang="ts">
  import { mobileMenuOpen } from "$lib/stores";
  import type { Component } from "svelte";

  interface MobileMenuProps {
    elements: Component[];
  }

  let { elements = [] }: MobileMenuProps = $props();
</script>

{#if $mobileMenuOpen}
  <div class="fixed inset-0 z-50 flex flex-col items-center justify-end">
    <!-- Backdrop -->
    <button
      class="absolute inset-0 bg-black/40 backdrop-blur-md transition-opacity"
      onclick={() => mobileMenuOpen.false()}
      aria-label="Close menu"
    ></button>

    <!-- Slide-up menu panel -->
    <div
      class="relative z-10 w-full max-w-md rounded-t-3xl bg-white/80 shadow-2xl ring-1 ring-black/5 backdrop-blur-xl transition-all dark:bg-stone-900/90 dark:ring-white/10"
    >
      <!-- Drag handle indicator -->
      <div
        class="mx-auto mt-3 mb-2 h-1.5 w-12 rounded-full bg-stone-300 dark:bg-stone-700"
      ></div>

      <nav class="px-6 pt-2 pb-6">
        <ul class="flex flex-col space-y-4">
          {#each elements as Element}
            <button
              class="w-full text-left text-base font-medium text-stone-800 transition-opacity hover:opacity-80 dark:text-stone-100"
              onclick={() => mobileMenuOpen.false()}
            >
              <Element />
            </button>
          {/each}
        </ul>
      </nav>
    </div>
  </div>
{/if}
