<script lang="ts">
  export let data: any;
  const articles:App.Article[] = data.articles;
  </script>

<h1 class="text-3xl font-bold">
    Welcome to Deuterium! 🚀
  </h1>
  {#if articles.length > 0}
    <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 my-6">
      {#each articles as article}
        <div class="bg-white shadow-md p-4 rounded-lg">
          <h2 class="text-xl font-bold">{article.title}</h2>
          <p class="text-gray-600">{@html article.processed_html}</p>
        </div>
      {/each}
    </div>
  {:else}
    <p>No articles found</p>
  {/if}
  
  <style lang="postcss">
    :global(html) {
      background-color: theme(colors.gray.100);
    }
  </style>