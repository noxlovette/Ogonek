<script lang="ts">
  let {
    priority = 3,
    disabled = false,
  }: {
    priority: number;
    disabled?: boolean;
  } = $props();

  const getPriorityConfig = (value: number) => {
    switch (value) {
      case 1:
        return {
          label: "High Priority",
          color: "text-red-600 dark:text-red-400",
          bgColor: "bg-red-500",
        };
      case 2:
        return {
          label: "Medium Priority",
          color: "text-orange-600 dark:text-orange-400",
          bgColor: "bg-orange-500",
        };
      case 3:
        return {
          label: "Low Priority",
          color: "text-green-600 dark:text-green-400",
          bgColor: "bg-green-500",
        };
      default:
        return {
          label: "Unknown Priority",
          color: "text-gray-600 dark:text-gray-400",
          bgColor: "bg-gray-500",
        };
    }
  };

  const config = $derived(getPriorityConfig(priority));

  const setPriority = (newPriority: number) => {
    if (!disabled) {
      priority = newPriority;
    }
  };
</script>

<div class="flex flex-col gap-3">
  <!-- Priority Label -->
  <div class="flex items-center justify-between">
    <p class={`text-sm font-medium ${config.color}`}>
      {config.label}
    </p>
    <span class={`text-sm font-bold ${config.color}`}>
      P{priority}
    </span>
  </div>

  <!-- Priority Buttons -->
  <div class="flex items-center justify-between">
    {#each [1, 2, 3] as level, index (index)}
      {@const levelConfig = getPriorityConfig(level)}
      <button
        type="button"
        onclick={() => setPriority(level)}
        {disabled}
        class={`
            focus:ring-cacao-500 h-4 w-4 rounded-full border-2 transition-all 
            duration-200 hover:scale-110 focus:ring-2 focus:ring-offset-2 focus:outline-none active:scale-95
            disabled:cursor-not-allowed disabled:hover:scale-100
            ${
              priority === level
                ? `${levelConfig.bgColor} border-white shadow-md dark:border-gray-800`
                : "border-gray-400 bg-gray-300 hover:bg-gray-400 dark:border-gray-500 dark:bg-gray-600 dark:hover:bg-gray-500"
            }
          `}
        aria-label={`Set priority to ${levelConfig.label}`}
      >
      </button>
    {/each}
  </div>
</div>

<input type="hidden" bind:value={priority} name="priority" />
