<script lang="ts">
    import { getContext } from 'svelte';

    const word = getContext('word');



</script>

<div class="flex  flex-col p-4 border-2 border-sand-900/20 rounded-lg">
<h2 class="text-xl">
    Random Word
</h2>
<div class="p-2">
<h3 class="italic">
    {word.word}
</h3>
<p class="text-base">
    {#if word.results}
    {word.results[0].definition}
    {:else}
    No definition found
    {/if}
</p>
</div>

</div>
