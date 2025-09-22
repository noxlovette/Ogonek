<script lang="ts">
  import { format, fromZonedTime, toZonedTime } from "date-fns-tz";
  import { isValid, parse } from "date-fns";
  import { HStack, VStack } from "..";
  import Input from "./Input.svelte";
  import Divider from "../toolbar/Divider.svelte";
  import type { ActionData } from "../../../../routes/(app)/[role=role]/calendar/[year]/[month]/[day]/[id]/edit/$types";

  let {
    dtstartTime,
    dtendTime,
    form,
  }: { dtstartTime: string; dtendTime?: string | null; form: ActionData } =
    $props();

  const userTimezone = Intl.DateTimeFormat().resolvedOptions().timeZone;

  let localDateString = $state("");
  let dtstartLocalTimeString = $state("");
  let dtendLocalTimeString = $state("");

  $effect(() => {
    if (isValid(new Date(dtstartTime))) {
      const localDate = toZonedTime(dtstartTime, userTimezone);
      localDateString = format(localDate, "yyyy-MM-dd");
      dtstartLocalTimeString = format(localDate, "HH:mm");
      if (dtendTime && isValid(new Date(dtendTime))) {
        const localEndDate = toZonedTime(dtendTime, userTimezone);
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

    if (dtendTime) {
      const dtendLocalDateTime = parse(
        dtendLocalDateTimeString,
        "yyyy-MM-dd HH:mm",
        new Date(),
      );

      const dtendISO = fromZonedTime(
        dtendLocalDateTime,
        userTimezone,
      ).toISOString();
      dtendTime = dtendISO;
    }

    const dtstartISO = fromZonedTime(localDateTime, userTimezone).toISOString();

    dtstartTime = dtstartISO;
  }
</script>

<HStack>
  <input type="hidden" name="dtstartTime" bind:value={dtstartTime} />
  <input type="hidden" name="dtendTime" bind:value={dtendTime} />
  <VStack>
    <Input
      type="date"
      name="date"
      labelName="Дата"
      onchange={handleInputChange}
      bind:value={localDateString}
    />
    <Divider />
    <Input
      type="time"
      name="startTime"
      labelName="Начало"
      onchange={handleInputChange}
      bind:value={dtstartLocalTimeString}
    />
    <Input
      type="time"
      name="endTime"
      labelName="Конец"
      onchange={handleInputChange}
      invalid={form?.dtend}
      invalidDescription="Проверьте время"
      bind:value={dtendLocalTimeString}
    />
  </VStack>
</HStack>
