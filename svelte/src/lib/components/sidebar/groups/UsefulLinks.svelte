<script lang="ts">
  import { H3 } from "$lib/components/typography";
  import {
    Forvo,
    Dictionary,
    Thesaurus,
  } from "$lib/components/sidebar/elements";
  import Group from "./Group.svelte";

  let elements = [Forvo, Dictionary, Thesaurus];
</script>

<Group>
  <H3>Useful Links</H3>
  <div class="flex-row space-y-2">
    {#each elements as Element, index (index)}
      <Element />
    {/each}
  </div>
</Group>
