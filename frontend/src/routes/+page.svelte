<script lang="ts">
	import TaskBox from "$lib/components/TaskBox.svelte";

  export let data: any;
  const tasks:App.Task[] = data.tasks;
  </script>

<h1 class="text-7xl font-bold">
    Welcome to Firelight
  </h1>
  {#if tasks.length > 0}
    <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 my-6">
      {#each tasks as task}
        <TaskBox task={task} />
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