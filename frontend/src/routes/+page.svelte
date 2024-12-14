<script lang="ts">
  export let data: any;
  const tasks:App.Task[] = data.tasks;
  </script>

<h1 class="text-3xl font-bold">
    Welcome to Deuterium! 🚀
  </h1>
  {#if tasks.length > 0}
    <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 my-6">
      {#each tasks as task}
        <div class="bg-white shadow-md p-4 rounded-lg">
          <h2 class="text-xl font-bold">{task.title}</h2>
          <p class="text-gray-600">{@html task.description}</p>
        </div>
      {/each}
    </div>
  {:else}
    <p>No tasks found</p>
  {/if}
  
  <style lang="postcss">
    :global(html) {
      background-color: theme(colors.gray.100);
    }
  </style>