<script lang="ts">
  import { Caption1 } from "../typography";

  const { priority = 3 } = $props();

  const getPriorityLabel = (priority: number): string => {
    switch (priority) {
      case 1:
        return "High";
      case 2:
        return "Medium";
      case 3:
        return "Low";
      default:
        return "Unknown";
    }
  };

  const getPriorityConfig = (priority: number) => {
    switch (priority) {
      case 1:
        return {
          color: "bg-red-500",
          textColor: "text-red-600 dark:text-red-400",
        };
      case 2:
        return {
          color: "bg-amber-500",
          textColor: "text-amber-600 dark:text-amber-400",
        };
      case 3:
        return {
          color: "bg-emerald-500",
          textColor: "text-emerald-600 dark:text-emerald-400",
        };
      default:
        return {
          color: "bg-gray-500",
          textColor: "text-gray-600 dark:text-gray-400",
        };
    }
  };

  const priorityLabel = getPriorityLabel(priority);
  const priorityConfig = getPriorityConfig(priority);
</script>

<div class="flex items-center gap-1">
  <span class={`h-2.5 w-2.5 rounded-full ${priorityConfig.color}`}></span>
  <Caption1 styling={priorityConfig.textColor}>
    {priorityLabel}
  </Caption1>
</div>
