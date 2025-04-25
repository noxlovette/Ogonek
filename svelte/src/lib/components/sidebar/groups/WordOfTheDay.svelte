<script lang="ts">
  import Group from "./Group.svelte";
  import { H2 } from "$lib/components";
  import { getContext } from "svelte";
  import type { Word } from "$lib/types";
  import Caption from "$lib/components/typography/Caption.svelte";

  const word = getContext<Promise<Word>>("word");
</script>

<Group>
  <H2>Word</H2>

  {#await word}
    <h3>Loading...</h3>
  {:then word}
    <h3 class="italic select-text">
      {word.word}
    </h3>

    {#if word.results}
      <p class="text-sm font-semibold">
        {word.results[0].definition}
      </p>
    {:else}
      <Caption>No definition found</Caption>
    {/if}
  {/await}
</Group>
