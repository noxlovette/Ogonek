<script lang="ts">
  const { priority = 3 } = $props();

  interface PriorityConfig {
    segments: number;
    color: string;
    bgColor: string;
  }

  const getPriorityConfig = (priority: number): PriorityConfig => {
    switch (priority) {
      case 1:
        return {
          segments: 3,
          color: "bg-red-500",
          bgColor: "bg-red-200 dark:bg-red-900",
        };
      case 2:
        return {
          segments: 2,
          color: "bg-orange-500",
          bgColor: "bg-orange-200 dark:bg-orange-900",
        };
      case 3:
        return {
          segments: 1,
          color: "bg-green-500",
          bgColor: "bg-green-200 dark:bg-green-900",
        };
      default:
        return {
          segments: 1,
          color: "bg-gray-500",
          bgColor: "bg-gray-200 dark:bg-gray-900",
        };
    }
  };

  const priorityConfig = getPriorityConfig(priority);

  // Generate an array of 3 segments with active/inactive status
  const segments = Array.from({ length: 3 }, (_, i) => ({
    active: i < priorityConfig.segments,
  }));
</script>

<div class="flex flex-shrink-0 flex-col items-end gap-1">
  <div class="flex gap-0.5">
    {#each segments as segment, index (index)}
      <div
        class={`h-4 w-1 rounded-sm  duration-200 ${
          segment.active ? priorityConfig.color : priorityConfig.bgColor
        }`}
      ></div>
    {/each}
  </div>
</div>
