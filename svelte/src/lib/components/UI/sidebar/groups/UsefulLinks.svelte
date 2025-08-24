<script lang="ts">
  import { H3 } from "$lib/components/typography";
  import {
    Forvo,
    Dictionary,
    Thesaurus,
  } from "$lib/components/UI/sidebar/elements";
  import Group from "./Group.svelte";
  import { m } from "$lib/paraglide/messages";

  let elements = [Forvo, Dictionary, Thesaurus];
</script>

<Group>
  <H3>{m.usefulLinks()}</H3>
  {#each elements as Element, index (index)}
    <Element />
  {/each}
</Group>
