<script lang="ts">
    import { getContext } from "svelte";

    const meta:App.Meta = getContext("meta");
    const article:App.Article = getContext("article");

    let selectedCategory:App.Category;
    selectedCategory = article.category;


</script>

<input type="hidden" name="category" bind:value={selectedCategory} />

<div>
    <select
          class="col-start-1 row-start-1 rounded border-2 border-stone-500 bg-transparent px-4 py-1 ring-0 focus:border-stone-500 focus:ring-0 "
          bind:value={selectedCategory}
        >
        {#each meta.categories as category}
          <option value={category.id}>{category.name}</option>
        {/each}
        </select>

</div>