<script lang="ts">
    import { getContext } from 'svelte';

    const word = getContext('word');



</script>

<div class="flex flex-col p-2 lg:p-3 xl:p-4 border-2 border-sand-900/20 rounded-lg">
<h2 class=" lg:text-lg xl:text-xl">
    Random Word
</h2>
<div class="p-2">
<h3 class="italic select-text">
    {word.word}
</h3>
<p class="text-xs lg:text-base select-text">
    {#if word.results}
    {word.results[0].definition}
    {:else}
    No definition found
    {/if}
</p>
</div>

</div>
