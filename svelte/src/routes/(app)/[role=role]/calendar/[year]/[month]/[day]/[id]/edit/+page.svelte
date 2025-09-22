<script lang="ts">
  import {
    BackButton,
    CancelButton,
    DeleteButton,
    Divider,
    HStack,
    Input,
    Merger,
    SaveButton,
    SectionBg,
    Title1,
    VStack,
    DateTimePicker,
    Optional,
    RecurrenceSelector,
  } from "$lib/components";
  import { enhance } from "$app/forms";
  import { enhanceForm } from "@noxlovette/svarog";

  const { data, form } = $props();
  const event = data.event;

  let showDescription = $state(false);
  let showLocation = $state(false);

  const student = event.attendees.filter(
    (attendee) => attendee.role !== "chair",
  )[0];
</script>

<svelte:head>
  <title>{event.title} • Календарь</title>
  <meta name="description" content={event.description || event.title} />
</svelte:head>
<form
  use:enhance={enhanceForm({
    messages: {
      redirect: "Изменения сохранены",
    },
    shouldUpdate: true,
    navigate: true,
  })}
  method="POST"
  action="?/update"
  class="flex w-full flex-col gap-3 md:gap-3 lg:gap-4"
>
  <BackButton />
  <VStack>
    <Title1>
      {event.title}
    </Title1>
    <Divider />
    <Merger>
      <DeleteButton />
      <CancelButton />
      <SaveButton />
    </Merger>
  </VStack>
  <SectionBg>
    <HStack>
      <DateTimePicker
        {form}
        dtstartTime={event.dtstartTime}
        dtendTime={event.dtendTime}
      />
      {#if !event.isRecurring}
        <RecurrenceSelector rrule={event.rrule} />
      {/if}
    </HStack>
  </SectionBg>
  <SectionBg>
    <HStack>
      <Input
        showLabel={false}
        placeholder="Выберите ученика"
        value={student?.userId}
        type="attendee"
        name="attendee"
      />
    </HStack>
  </SectionBg>
  {#if event.location || showLocation}
    <SectionBg>
      <Input
        placeholder="Или ссылка на видеозвонок"
        name="location"
        labelName="Локация"
        value={event.location}
      />
    </SectionBg>
  {:else}
    <Optional bind:toggle={showLocation}>Добавить локацию / Ссылку</Optional>
  {/if}
  {#if event.description || showDescription}
    <SectionBg>
      <Input
        type="textarea"
        value={event.description}
        name="description"
        labelName="Описание"
      ></Input>
    </SectionBg>
  {:else}
    <Optional bind:toggle={showDescription}>Добавить описание</Optional>
  {/if}
  <Input type="scope" name="scope" labelName="Охват изменений" />
</form>
