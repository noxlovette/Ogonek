<script>
  import { Footnote, Headline } from "$lib/components/typography";
  import { m } from "$lib/paraglide/messages";
  import { HStack, VStack } from "../layout";
  let width = $state(0);
  const isMobile = $derived(width < 768);
</script>

<svelte:window bind:innerWidth={width} />
<footer
  class="mt-30 flex w-full flex-col items-center justify-center border-t border-stone-200 px-1 py-3 text-xs md:mt-12 md:px-6 md:text-base lg:px-12 dark:border-stone-700"
>
  {#if isMobile}
    <HStack styling="items-center">
      <Footnote>
        <a href="/contact" class="hover:text-accent">{m.contact()}</a>
      </Footnote>

      <Footnote>
        <a href="/terms" class="hover:text-accent">{m.terms()}</a>
      </Footnote>
      <Footnote>
        <a href="/privacy-policy" class="hover:text-accent">{m.privacy()}</a>
      </Footnote>
      <Footnote styling="mt-10">
        © 2024-2025 {m.danilaVolkov()}
      </Footnote>
    </HStack>
  {:else}
    <VStack>
      <Footnote>
        <a href="/contact" class="hover:text-accent">{m.contact()}</a>
      </Footnote>

      <Footnote>
        <a href="/terms" class="hover:text-accent">{m.terms()}</a>
      </Footnote>
      <Footnote>
        <a href="/privacy-policy" class="hover:text-accent">{m.privacy()}</a>
      </Footnote>
      <Footnote styling="ml-10">
        © 2024-2025 {m.danilaVolkov()}
      </Footnote>
    </VStack>
  {/if}
</footer>
