<script lang="ts">
  import { format, fromZonedTime, toZonedTime } from "date-fns-tz";
  import {
    isValid,
    parse,
    addMinutes,
    differenceInMinutes,
    addDays,
  } from "date-fns";
  import { HStack, VStack } from "..";
  import Input from "./Input.svelte";
  import Divider from "../toolbar/Divider.svelte";
  import type { ActionData as EditActionData } from "../../../../routes/(app)/[role=role]/calendar/[year]/[month]/[day]/[id]/edit/$types";
  import type { ActionData as NewActionData } from "../../../../routes/(app)/[role=role]/calendar/[year]/[month]/[day]/new/$types";

  let {
    dtstartTime,
    dtendTime,
    form,
  }: {
    dtstartTime: string;
    dtendTime?: string | null;
    form: EditActionData | NewActionData;
  } = $props();

  const userTimezone = Intl.DateTimeFormat().resolvedOptions().timeZone;

  // État local pour les inputs
  let localDateString = $state("");
  let dtstartLocalTimeString = $state("");
  let dtendLocalTimeString = $state("");

  // Stockage de la durée en minutes
  let storedDurationMinutes = $state(60); // Default 1 heure
  let isInitialized = $state(false);

  // Initialisation des valeurs et calcul de la durée initiale
  $effect(() => {
    if (isValid(new Date(dtstartTime))) {
      const localStartDate = toZonedTime(dtstartTime, userTimezone);
      localDateString = format(localStartDate, "yyyy-MM-dd");
      dtstartLocalTimeString = format(localStartDate, "HH:mm");

      if (dtendTime && isValid(new Date(dtendTime))) {
        const localEndDate = toZonedTime(dtendTime, userTimezone);
        dtendLocalTimeString = format(localEndDate, "HH:mm");

        // Calculer et stocker la durée initiale
        if (!isInitialized) {
          const startDateTime = new Date(dtstartTime);
          const endDateTime = new Date(dtendTime);
          storedDurationMinutes = differenceInMinutes(
            endDateTime,
            startDateTime,
          );
          isInitialized = true;
        }
      } else if (!isInitialized) {
        // Si pas de dtend, utiliser la durée par défaut
        isInitialized = true;
        updateEndTimeFromDuration();
      }
    } else {
      console.warn("Date de début invalide");
    }
  });

  function updateEndTimeFromDuration() {
    if (!localDateString || !dtstartLocalTimeString) return;

    const localStartDateTimeString = `${localDateString} ${dtstartLocalTimeString}`;
    const localStartDateTime = parse(
      localStartDateTimeString,
      "yyyy-MM-dd HH:mm",
      new Date(),
    );

    if (!isValid(localStartDateTime)) return;

    // Ajouter la durée stockée
    const localEndDateTime = addMinutes(
      localStartDateTime,
      storedDurationMinutes,
    );

    // Gestion du passage au jour suivant
    const endDateString = format(localEndDateTime, "yyyy-MM-dd");
    dtendLocalTimeString = format(localEndDateTime, "HH:mm");

    // Conversion vers ISO pour le backend
    const dtstartISO = fromZonedTime(
      localStartDateTime,
      userTimezone,
    ).toISOString();
    const dtendISO = fromZonedTime(
      localEndDateTime,
      userTimezone,
    ).toISOString();

    dtstartTime = dtstartISO;
    dtendTime = dtendISO;

    // Log pour debug en dev
    if (endDateString !== localDateString) {
      console.log(`⚠️  L'événement se termine le lendemain (${endDateString})`);
    }
  }

  function handleStartTimeChange() {
    // Quand dtstart change, on applique la durée stockée
    updateEndTimeFromDuration();
  }

  function handleEndTimeChange() {
    if (!localDateString || !dtstartLocalTimeString || !dtendLocalTimeString)
      return;

    const localStartDateTimeString = `${localDateString} ${dtstartLocalTimeString}`;
    const localStartDateTime = parse(
      localStartDateTimeString,
      "yyyy-MM-dd HH:mm",
      new Date(),
    );

    // Parse end time - peut être le lendemain
    let localEndDateTime = parse(
      `${localDateString} ${dtendLocalTimeString}`,
      "yyyy-MM-dd HH:mm",
      new Date(),
    );

    // Si l'heure de fin est antérieure à l'heure de début, c'est le lendemain
    if (localEndDateTime <= localStartDateTime) {
      localEndDateTime = addDays(localEndDateTime, 1);
      console.log("🔄 Heure de fin ajustée au lendemain");
    }

    if (!isValid(localStartDateTime) || !isValid(localEndDateTime)) return;

    // Recalculer et stocker la nouvelle durée
    storedDurationMinutes = differenceInMinutes(
      localEndDateTime,
      localStartDateTime,
    );

    // Conversion vers ISO
    const dtstartISO = fromZonedTime(
      localStartDateTime,
      userTimezone,
    ).toISOString();
    const dtendISO = fromZonedTime(
      localEndDateTime,
      userTimezone,
    ).toISOString();

    dtstartTime = dtstartISO;
    dtendTime = dtendISO;
  }

  function handleDateChange() {
    // Quand la date change, on reapplique la durée
    updateEndTimeFromDuration();
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
      onchange={handleDateChange}
      bind:value={localDateString}
    />

    <Divider />

    <Input
      type="time"
      name="startTime"
      labelName="Начало"
      onchange={handleStartTimeChange}
      bind:value={dtstartLocalTimeString}
    />

    <Input
      type="time"
      name="endTime"
      labelName="Конец"
      onchange={handleEndTimeChange}
      invalid={form?.dtend}
      invalidDescription="Проверьте время"
      bind:value={dtendLocalTimeString}
    />
  </VStack>

  <!-- Debug info en dev mode -->
  {#if import.meta.env.DEV}
    <div class="mt-2 text-xs text-stone-500">
      Durée: {Math.floor(storedDurationMinutes / 60)}h {storedDurationMinutes %
        60}min
    </div>
  {/if}
</HStack>
