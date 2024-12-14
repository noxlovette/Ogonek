<script lang="ts">
    import { getContext } from "svelte";

    const article:App.Article = getContext('article');


    let selectedDifficulty:number = article.difficulty || 1;
</script>

<input type="range" name="difficulty" min="0" max="10" step="1" bind:value={selectedDifficulty} />