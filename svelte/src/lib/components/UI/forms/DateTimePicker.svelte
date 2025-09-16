<script lang="ts">
  import { format, fromZonedTime, toZonedTime } from "date-fns-tz";
  import { isValid, parse } from "date-fns";
  import { Divider, HStack, VStack } from "..";
  import Input from "./Input.svelte";

  let { dtstart, dtend }: { dtstart: string; dtend?: string | null } = $props();

  const userTimezone = Intl.DateTimeFormat().resolvedOptions().timeZone;

  let localDateString = $state("");
  let dtstartLocalTimeString = $state("");
  let dtendLocalTimeString = $state("");

  $effect(() => {
    if (isValid(new Date(dtstart))) {
      const localDate = toZonedTime(dtstart, userTimezone);
      localDateString = format(localDate, "yyyy-MM-dd");
      dtstartLocalTimeString = format(localDate, "HH:mm");
      if (dtend && isValid(new Date(dtend))) {
        const localEndDate = toZonedTime(dtend, userTimezone);
        dtendLocalTimeString = format(localEndDate, "HH:mm");
      }
    } else {
      console.log("invalid date");
    }
  });

  function handleInputChange() {
    const localDateTimeString = `${localDateString} ${dtstartLocalTimeString}`;

    const dtendLocalDateTimeString = `${localDateString} ${dtendLocalTimeString}`;
    const localDateTime = parse(
      localDateTimeString,
      "yyyy-MM-dd HH:mm",
      new Date(),
    );

    const dtendLocalDateTime = parse(
      dtendLocalDateTimeString,
      "yyyy-MM-dd HH:mm",
      new Date(),
    );

    const dtstartISO = fromZonedTime(localDateTime, userTimezone).toISOString();

    const dtendISO = fromZonedTime(
      dtendLocalDateTime,
      userTimezone,
    ).toISOString();

    dtstart = dtstartISO;
    dtend = dtendISO;
  }
</script>

<HStack>
  <input type="hidden" name="dtstart" bind:value={dtstart} />
  <input type="hidden" name="dtend" bind:value={dtend} />
  <VStack>
    <Input
      type="date"
      name="Дата"
      onchange={handleInputChange}
      bind:value={localDateString}
    />
    <Input
      type="time"
      name="Начало"
      onchange={handleInputChange}
      bind:value={dtstartLocalTimeString}
    />
    <Input
      type="time"
      name="Конец"
      onchange={handleInputChange}
      bind:value={dtendLocalTimeString}
    />
  </VStack>
</HStack>
