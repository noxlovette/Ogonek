<script lang="ts">
  import { getLocaleFromCookie, getVideoCallService } from "$lib/utils";
  import { MapPin, Share, Video } from "lucide-svelte";
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
  } from "$lib/components";
  import { page } from "$app/state";
  import RRule from "$lib/components/UI/RRule.svelte";

  const { data } = $props();
  const event = data.event;

  let isSharing = $state(false);

  async function shareEvent() {
    isSharing = true;
    try {
      if (navigator.share) {
        await navigator.share({
          title: event.title,
          text: event.description || "",
          url: window.location.href,
        });
      } else {
        // Fallback: copy to clipboard
        await navigator.clipboard.writeText(window.location.href);
        alert("Link copied to clipboard!");
      }
    } catch (error) {
      console.log("Share cancelled or failed");
    } finally {
      isSharing = false;
    }
  }

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
      <UniButton Icon={Share} onclick={shareEvent} disable={isSharing} />
    </Merger>
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
            <UniButton href={event.location} Icon={Video}>
              Присоединиться к звонку
            </UniButton>
          {:else}
            <UniButton
              href={`https://yandex.com/maps/?text=${encodeURIComponent(event.location)}`}
              Icon={MapPin}
            >
              Посмотреть на карте
            </UniButton>
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
