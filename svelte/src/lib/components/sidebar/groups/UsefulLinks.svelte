<script lang="ts">
  import H2 from "$lib/components/typography/H2.svelte";
  import {
    Forvo,
    Dictionary,
    Thesaurus,
  } from "$lib/components/sidebar/elements";
  import Group from "./Group.svelte";
  import { sidebar } from "$lib/stores";

  let elements = [Forvo, Dictionary, Thesaurus];
</script>

<Group>
  <H2>Useful Links</H2>
  <div class={$sidebar ? "flex-row py-2" : "flex-col"}>
    {#each elements as Element}
      <Element />
    {/each}
  </div>
</Group>
