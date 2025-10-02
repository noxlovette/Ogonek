<script lang="ts">
  import { getLocaleFromCookie, getVideoCallService } from "$lib/utils";
  import { MapPin, Share, Video } from "@lucide/svelte";
  import {
    BackButton,
    Body,
    Callout,
    Caption1,
    Divider,
    Headline,
    HStack,
    Merger,
    SectionBg,
    Title1,
    Title2,
    Title3,
    UniButton,
    VStack,
    EditButton,
    RRule,
  } from "$lib/components";
  import { page } from "$app/state";
  import { m } from "$lib/paraglide/messages.js";

  const { data } = $props();
  const event = data.event;

  const start = new Date(event.dtstartTime);
  const end = event.dtendTime ? new Date(event.dtendTime) : null;

  const dateOptions: Intl.DateTimeFormatOptions = {
    weekday: "short",
    month: "short",
    day: "numeric",
  };

  const timeOptions: Intl.DateTimeFormatOptions = {
    hour: "numeric",
    minute: "2-digit",
    hour12: false,
  };
  const locale = getLocaleFromCookie();

  const videoCallService = event.location
    ? getVideoCallService(event.location)
    : null;
</script>

<svelte:head>
  <title>{event.title} • Календарь</title>
  <meta name="description" content={event.description || event.title} />
</svelte:head>

<HStack>
  <BackButton />
  <VStack>
    <Title1 styling={event.status === "cancelled" ? "line-through" : ""}>
      {page.params.role === "t" ? event.title : "Занятие"}
    </Title1>
    <Divider />
    <Merger>
      <EditButton href="{event.id}/edit" />
    </Merger>
  </VStack>

  <HStack>
    <VStack>
      <Title2>
        {start.toLocaleTimeString(locale, timeOptions)}
        {#if end}
          -
          {end.toLocaleTimeString(locale, timeOptions)}
        {/if}
      </Title2>
      <Title3>
        {start.toLocaleDateString(locale, dateOptions)}
      </Title3>
    </VStack>

    {#if event.rrule}
      <RRule rrule={event.rrule} />
    {/if}
  </HStack>

  <SectionBg>
    {#if event.attendees}
      {#each event.attendees as attendee, index}
        <Headline>
          {attendee.name}
        </Headline>
        <Caption1>
          {attendee.email}
        </Caption1>
      {/each}
    {/if}
    {#if event.location}
      <VStack>
        <Callout styling="truncate">
          {#if videoCallService}
            {videoCallService}
          {:else}
            {event.location}
          {/if}
        </Callout>
        <Divider />
        <Merger>
          {#if videoCallService}
            <UniButton
              content={m.mealy_zesty_pony_grow()}
              href={event.location}
              Icon={Video}
            ></UniButton>
          {:else}
            <UniButton
              content="Посмотреть на карте"
              href={`https://yandex.ru/maps/?text=${encodeURIComponent(event.location)}`}
              Icon={MapPin}
            ></UniButton>
          {/if}
        </Merger>
      </VStack>
    {/if}
  </SectionBg>

  {#if event.description}
    <SectionBg>
      <Body>
        {event.description}
      </Body>
    </SectionBg>
  {/if}
</HStack>
