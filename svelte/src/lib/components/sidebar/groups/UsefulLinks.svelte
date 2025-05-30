<script lang="ts">
  import H2 from "$lib/components/typography/H2.svelte";
  import {
    Forvo,
    Dictionary,
    Thesaurus,
  } from "$lib/components/sidebar/elements";
  import Group from "./Group.svelte";

  let elements = [Forvo, Dictionary, Thesaurus];
</script>

<Group>
  <H2>Useful Links</H2>
  <div class="flex-row space-y-2">
    {#each elements as Element, index (index)}
      <Element />
    {/each}
  </div>
</Group>
