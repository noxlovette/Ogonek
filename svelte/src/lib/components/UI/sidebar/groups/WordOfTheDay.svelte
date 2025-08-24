<script lang="ts">
  import Group from "./Group.svelte";
  import { Title3 } from "$lib/components";
  import { getContext } from "svelte";
  import type { Word } from "$lib/types";
  import Caption1 from "$lib/components/typography/Caption1.svelte";
  import { m } from "$lib/paraglide/messages";

  const word = getContext<Promise<Word>>("word");
</script>

<Group>
  <Title3>{m.neat_sound_ocelot_belong()}</Title3>

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
      <Caption1>No definition found</Caption1>
    {/if}
  {/await}
</Group>
