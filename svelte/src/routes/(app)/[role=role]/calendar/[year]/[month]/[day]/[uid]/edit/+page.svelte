<script lang="ts">
  import { parseRRuleToText, getLocaleFromCookie } from "$lib/utils";
  import { ChevronLeft, MapPin, Share } from "lucide-svelte";
  import {
    Body,
    Callout,
    DeleteButton,
    Divider,
    HStack,
    Input,
    Merger,
    SaveButton,
    Title1,
    Title2,
    Title3,
    UniButton,
    VStack,
  } from "$lib/components";
  import EditButton from "$lib/components/UI/forms/buttons/EditButton.svelte";
  import DateTimePicker from "$lib/components/UI/forms/DateTimePicker.svelte";

  const { data } = $props();
  const event = data.event;

  const locale = getLocaleFromCookie();
</script>

<svelte:head>
  <title>{event.summary} • Календарь</title>
  <meta name="description" content={event.description || event.summary} />
</svelte:head>
<DateTimePicker value={event.dtstart} label="Blabla" />
<HStack>
  <VStack>
    <Merger>
      <UniButton onclick={() => window.history.back()} Icon={ChevronLeft}
      ></UniButton>
    </Merger>
  </VStack>
  <VStack>
    <Title1 styling={event.status === "cancelled" ? "line-through" : ""}>
      {event.summary}
    </Title1>
    <Divider />
    <Merger>
      <DeleteButton confirmText="Delete Event" confirmTitle="Delete Event"
      ></DeleteButton>
      <SaveButton />
    </Merger>
  </VStack>
  <VStack>
    {event.dtstart}
    <Input
      type="datetime"
      value={event.dtstart}
      name="dtstart"
      labelName="Начало"
    />
    <Input type="datetime" value={event.dtend} name="dtend" labelName="Конец" />
  </VStack>

  {#if event.location}
    <div class="bg-default ring-default rounded-xl p-2">
      <VStack>
        <Callout>
          {event.location}
        </Callout>

        <Divider></Divider>
        <Merger>
          <UniButton
            href={"https://yandex.com/maps/?text=" +
              encodeURIComponent(event.location)}
            Icon={MapPin}>Посмотреть на карте</UniButton
          >
        </Merger>
      </VStack>
    </div>
  {/if}

  <!-- Description -->
  {#if event.description}
    <div class="bg-default ring-default rounded-xl p-2">
      <Body>
        {event.description}
      </Body>
    </div>
  {/if}

  {#if event.attendees?.length}
    <div class="bg-default ring-default rounded-xl p-2">
      <HStack>
        {#each event.attendees as attendee}
          <div class="flex items-center gap-3">
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
          </div>
        {/each}
      </HStack>
    </div>
  {/if}
</HStack>
