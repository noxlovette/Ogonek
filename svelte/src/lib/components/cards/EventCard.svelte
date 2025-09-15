<script lang="ts">
  import { page } from "$app/state";
  import type { CalendarEvent } from "$lib/types/api/calendar";

  const { event }: { event: CalendarEvent } = $props();
</script>

<a
  class="bg-default ring-default flex items-center justify-between gap-3 rounded-2xl p-2 shadow-md"
  href="{page.params.role}/calendar/{page.params.iso}/{event.uid}"
>
  <div class="flex flex-1 flex-col overflow-hidden">
    <p
      class="truncate text-sm font-semibold text-stone-600 capitalize dark:text-stone-400"
    >
      {event.summary}
      <span class="ml-1 font-normal text-stone-600">
        {event.description}
      </span>
    </p>
  </div>
</a>
