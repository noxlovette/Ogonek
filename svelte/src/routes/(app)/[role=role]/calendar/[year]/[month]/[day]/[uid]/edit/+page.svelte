<script lang="ts">
  import { getLocaleFromCookie } from "$lib/utils";
  import { ChevronLeft, Plus, X } from "lucide-svelte";
  import {
    Body,
    Callout,
    CancelButton,
    Divider,
    HStack,
    Input,
    Merger,
    SaveButton,
    SectionBg,
    Title1,
    UniButton,
    VStack,
  } from "$lib/components";
  import DateTimePicker from "$lib/components/UI/forms/DateTimePicker.svelte";
  import { enhance } from "$app/forms";
  import Optional from "$lib/components/UI/forms/Optional.svelte";
  import type { EventAttendeeCreate } from "$lib/types/api/calendar.js";
  import RecurrenceSelector from "$lib/components/UI/forms/RecurrenceSelector.svelte";

  const { data } = $props();
  const event = data.event;

  let showDescription = $state(false);
  let showLocation = $state(false);

  let newAttendees: EventAttendeeCreate[] = $state([]);

  let updatedAttendees = $state(event.attendees);
  function addAttendee() {
    newAttendees = [
      ...newAttendees,
      {
        name: null,
        email: "",
      },
    ];
  }
</script>

<svelte:head>
  <title>{event.summary} • Календарь</title>
  <meta name="description" content={event.description || event.summary} />
</svelte:head>
<form
  use:enhance
  method="POST"
  class="flex w-full flex-col gap-3 md:gap-3 lg:gap-4"
>
  <VStack>
    <Merger>
      <UniButton
        type="button"
        onclick={() => window.history.back()}
        Icon={ChevronLeft}
      ></UniButton>
    </Merger>
  </VStack>
  <VStack>
    <Title1 styling={event.status === "cancelled" ? "line-through" : ""}>
      {event.summary}
    </Title1>
    <Divider />
    <Merger>
      <CancelButton />
      <SaveButton />
    </Merger>
  </VStack>
  <SectionBg>
    <HStack>
      <DateTimePicker dtstart={event.dtstart} dtend={event.dtend} />
      <RecurrenceSelector rrule={event.rrule} />
    </HStack>
  </SectionBg>

  {#if event.location || showLocation}
    <SectionBg>
      <Input name="location" labelName="Локация" value={event.location} />
    </SectionBg>
  {:else}
    <Optional bind:toggle={showLocation}>Добавить локацию</Optional>
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

  {#if updatedAttendees.length}
    <SectionBg>
      <HStack>
        {#each updatedAttendees as attendee, index}
          <VStack>
            <div
              class="bg-accent/50 dark:bg-accent/50 flex size-8 items-center justify-center rounded-full"
            >
              <Callout>
                {attendee.name?.charAt(0).toUpperCase()}
              </Callout>
            </div>

            <Body>
              {attendee.name}
            </Body>
            <Divider></Divider>
            <Merger>
              <UniButton
                Icon={X}
                onclick={() =>
                  (updatedAttendees = updatedAttendees.filter(
                    (_, i) => i !== index,
                  ))}
              />
            </Merger>
          </VStack>
          <input
            type="hidden"
            name={`old-attendees[${index}]`}
            value={attendee.email}
          />
        {/each}
        {#each newAttendees as _, index}
          <Input
            showLabel={false}
            placeholder="Выберите ученика"
            type="attendee"
            name={`new-attendees[${index}]`}
          />
        {/each}
        <Merger>
          <UniButton Icon={Plus} onclick={addAttendee} iconOnly={false}
            >Добавить участника</UniButton
          >
        </Merger>
      </HStack>
    </SectionBg>
  {/if}
</form>
