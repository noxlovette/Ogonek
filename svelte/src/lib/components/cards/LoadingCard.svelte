<script lang="ts">
  import CardClickable from "./CardClickable.svelte";
</script>

<CardClickable href=".">
  <div class="mb-4 flex items-start justify-between">
    <div class="flex-1">
      <div
        class="mb-2 h-6 w-3/4 animate-pulse rounded bg-stone-200 dark:bg-stone-800"
      ></div>
      <div
        class="h-5 w-20 animate-pulse rounded-full bg-stone-200 dark:bg-stone-800"
      ></div>
    </div>
    <div
      class="ml-4 h-8 w-8 animate-pulse rounded bg-stone-200 dark:bg-stone-800"
    ></div>
  </div>
</CardClickable>
