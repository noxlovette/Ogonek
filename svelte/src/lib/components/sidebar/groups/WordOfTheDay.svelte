<script lang="ts">
  import Group from "./Group.svelte";
  import { H2 } from "$lib/components";
  import { getContext } from "svelte";
  import type { Word } from "$lib/types";

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
    <p class="text-xs select-text lg:text-base">
      {#if word.results}
        {word.results[0].definition}
      {:else}
        No definition found
      {/if}
    </p>
  {/await}
</Group>
