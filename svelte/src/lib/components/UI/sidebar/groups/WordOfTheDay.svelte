<script lang="ts">
  import Group from "./Group.svelte";
  import { H3 } from "$lib/components";
  import { getContext } from "svelte";
  import type { Word } from "$lib/types";
  import Caption from "$lib/components/typography/Caption.svelte";
  import { m } from "$lib/paraglide/messages";

  const word = getContext<Promise<Word>>("word");
</script>

<Group>
  <H3>{m.neat_sound_ocelot_belong()}</H3>

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
