<script lang="ts">
  import { sortBy, sortOrder } from "$lib/stores";
  import type { SortBy, SortOrder } from "$lib/types";

  const sortFields: { value: SortBy; label: string }[] = [
    { value: "created_at", label: "Created At" },
    { value: "updated_at", label: "Updated At" },
    { value: "title", label: "Title" },
    { value: "due_date", label: "Due Date" },
  ];

  const sortOrderFields: { value: SortOrder; label: string }[] = [
    { value: "asc", label: "Восход" },
    { value: "desc", label: "Нисход" },
  ];
</script>

<select name="sort" bind:value={$sortBy}>
  {#each sortFields as field}
    <option value={field.value}>{field.label}</option>
  {/each}
</select>

<select name="order" bind:value={$sortOrder}>
  {#each sortOrderFields as order}
    <option value={order.value}>{order.label}</option>
  {/each}
</select>
