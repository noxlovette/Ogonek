<script lang="ts">
  import {
    parseRRuleToText,
    getLocaleFromCookie,
    isVideoCallUrl,
  } from "$lib/utils";
  import { ChevronLeft, MapPin, Share, Video } from "lucide-svelte";
  import {
    BackButton,
    Body,
    Callout,
    Caption1,
    DeleteButton,
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
  } from "$lib/components";
  import EditButton from "$lib/components/UI/forms/buttons/EditButton.svelte";
  import { page } from "$app/state";
  import Optional from "$lib/components/UI/forms/Optional.svelte";

  const { data } = $props();
  const event = data.event;

  let isSharing = $state(false);

  async function shareEvent() {
    isSharing = true;
    try {
      if (navigator.share) {
        await navigator.share({
          title: event.summary,
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

  const start = new Date(event.dtstart);
  const end = event.dtend ? new Date(event.dtend) : null;

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
</script>

<svelte:head>
  <title>{event.summary} • Календарь</title>
  <meta name="description" content={event.description || event.summary} />
</svelte:head>

<HStack>
  <BackButton />
  <VStack>
    <Title1 styling={event.status === "cancelled" ? "line-through" : ""}>
      {page.params.role === "t" ? event.summary : "Занятие"}
    </Title1>
    <Divider />
    <Merger>
      <UniButton Icon={Share} onclick={shareEvent} disable={isSharing} />
    </Merger>
    <Merger>
      <DeleteButton confirmText="Delete Event" confirmTitle="Delete Event"
      ></DeleteButton>
      <EditButton href="{event.uid}/edit" />
    </Merger>
  </VStack>

  <HStack>
    {#if !event.allDay}
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
    {:else}
      <Title2>Весь день</Title2>
    {/if}
    {#if event.rrule}
      <p class="text-stone-700 dark:text-stone-300">
        Повторяется {parseRRuleToText(event.rrule)}
      </p>
    {/if}
  </HStack>

  <SectionBg>
    {#each event.attendees as attendee, index}
      <Headline>
        {attendee.name}
      </Headline>
      <Caption1>
        {attendee.email}
      </Caption1>
    {:else}
      <Optional>Пригласить ученика</Optional>
    {/each}
  </SectionBg>

  {#if event.location}
    <SectionBg>
      <VStack>
        <Callout>
          {event.location}
        </Callout>
        <Divider />
        <Merger>
          {#if isVideoCallUrl(event.location)}
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
    </SectionBg>
  {/if}

  {#if event.description}
    <SectionBg>
      <Body>
        {event.description}
      </Body>
    </SectionBg>
  {/if}

  {#if event.organiserName}
    <SectionBg>
      <Body>
        Организатор: {event.organiserName}
      </Body>
    </SectionBg>
  {/if}
</HStack>
