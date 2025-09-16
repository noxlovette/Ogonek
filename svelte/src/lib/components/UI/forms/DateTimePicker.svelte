<script lang="ts">
  import { format, fromZonedTime, toZonedTime } from "date-fns-tz";
  import { isValid, parse } from "date-fns";

  let { value, label, required = false, disabled = false } = $props();

  const userTimezone = Intl.DateTimeFormat().resolvedOptions().timeZone;

  let localDateString = $state("");
  let localTimeString = $state("");

  $effect(() => {
    console.log(value);
    if (isValid(new Date(value))) {
      const localDate = toZonedTime(value, userTimezone);
      localDateString = format(localDate, "yyyy-MM-dd");
      localTimeString = format(localDate, "HH:mm");
    } else {
      console.log("invalid");
      resetToEmpty();
    }
  });
  function resetToEmpty() {
    localDateString = "";
    localTimeString = "";
  }

  function handleInputChange() {
    if (!localDateString && !localTimeString) {
      value = null;
      return;
    }

    try {
      const localDateTimeString = `${localDateString} ${localTimeString}`;
      const localDateTime = parse(
        localDateTimeString,
        "yyyy-MM-dd HH:mm",
        new Date(),
      );

      if (!isValid(localDateTime)) {
        throw new Error("Invalid date/time format");
      }

      // Convert to UTC
      const utcDateTime = fromZonedTime(localDateTime, userTimezone);
      const utcIsoString = utcDateTime.toISOString();

      value = utcIsoString;
    } catch (error) {
      console.error("DateTime parsing error:", error);
    }
  }

  function setToNow() {
    const utcNow = new Date().toISOString();

    value = utcNow;
  }

  function clearInput() {
    value = null;
  }
</script>

<div class="space-y-2">
  <div class="flex items-center justify-between">
    <div class="flex gap-2">
      <button
        type="button"
        onclick={setToNow}
        class="text-accent-600 hover:text-accent-700 text-xs font-medium"
        {disabled}
      >
        Now
      </button>

      {#if value}
        <button
          type="button"
          onclick={clearInput}
          class="text-xs text-stone-500 hover:text-stone-600"
          {disabled}
        >
          Clear
        </button>
      {/if}
    </div>
  </div>

  <div class="grid grid-cols-2 gap-3">
    <div>
      <input
        type="date"
        bind:value={localDateString}
        onchange={handleInputChange}
        class="focus:border-accent-500 focus:ring-accent-500 block w-full rounded-md
               border-stone-300 shadow-sm
               disabled:bg-stone-50 disabled:text-stone-500
               "
        {required}
        {disabled}
      />
    </div>

    <div>
      <input
        type="time"
        bind:value={localTimeString}
        onchange={handleInputChange}
        class="focus:border-accent-500 focus:ring-accent-500 block w-full rounded-md
               border-stone-300 shadow-sm
               disabled:bg-stone-50 disabled:text-stone-500
               "
        {required}
        {disabled}
      />
    </div>
  </div>
</div>
