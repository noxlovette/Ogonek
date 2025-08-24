<script lang="ts">
  import {
    Forvo,
    Dictionary,
    Thesaurus,
  } from "$lib/components/UI/sidebar/elements";

  let elements = [Forvo, Dictionary, Thesaurus];
</script>

{#each elements as Element, index (index)}
  <Element />
{/each}
