<script lang="ts">
  const { rows = 8 } = $props();

  const columns = 5;
</script>

<div class="bg-default ring-default overflow-hidden rounded-lg">
  <div class="grid grid-cols-5 gap-4 bg-stone-200 px-6 py-4 dark:bg-stone-800">
    {#each Array(columns) as _, i (i)}
      <div
        class="h-4 animate-pulse rounded bg-stone-50 dark:bg-stone-950"
        style="width:{i === 0 ? '75%' : i === columns - 1 ? '50%' : '66%'};"
      ></div>
    {/each}
  </div>
  <div class="divide-y divide-stone-100 dark:divide-stone-700">
    {#each Array(rows) as _, rowIndex (rowIndex)}
      <div class="px-6 py-4 transition-colors">
        <div
          class="grid grid-cols-5
            items-center gap-4"
        >
          {#each Array(columns) as _, colIndex (colIndex)}
            {@const widths = ["w-full", "w-5/6", "w-4/5", "w-3/4", "w-2/3"]}
            {@const randomWidth =
              widths[Math.floor(Math.random() * widths.length)]}
            <div
              class="h-4 animate-pulse rounded bg-stone-200 dark:bg-stone-950 {randomWidth}"
            ></div>
          {/each}
        </div>
      </div>
    {/each}
  </div>

  <!-- Pagination skeleton -->
  <div class="bg-default px-6 py-4">
    <div class="flex items-center justify-between">
      <div
        class="h-4 w-32 animate-pulse rounded bg-stone-200 dark:bg-stone-950"
      ></div>
      <div class="flex items-center space-x-2">
        <div
          class="h-8 w-8 animate-pulse rounded bg-stone-200 dark:bg-stone-950"
        ></div>
        <div
          class="h-8 w-8 animate-pulse rounded bg-stone-200 dark:bg-stone-950"
        ></div>
        <div
          class="h-8 w-8 animate-pulse rounded bg-stone-200 dark:bg-stone-950"
        ></div>
        <div
          class="h-8 w-8 animate-pulse rounded bg-stone-200 dark:bg-stone-950"
        ></div>
      </div>
    </div>
  </div>
</div>
