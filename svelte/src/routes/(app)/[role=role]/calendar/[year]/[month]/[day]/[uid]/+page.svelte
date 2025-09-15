<script lang="ts">
  import { page } from "$app/state";
  import { goto } from "$app/navigation";
  import { parseRRuleToText, getLocaleFromCookie } from "$lib/utils";
  import {
    ChevronLeft,
    MapPin,
    Clock,
    Users,
    Calendar,
    Edit,
    Trash2,
    Share,
  } from "lucide-svelte";
  import {
    Body,
    Callout,
    Caption1,
    DeleteButton,
    Divider,
    Headline,
    HStack,
    Merger,
    Title1,
    Title2,
    Title3,
    UniButton,
    VStack,
  } from "$lib/components";
  import EditButton from "$lib/components/UI/forms/buttons/EditButton.svelte";

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
  const endDate = end ? new Date(end.getTime() - 24 * 60 * 60 * 1000) : null;

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
        <!-- You'll need a helper to parse RRULE into human readable text -->
        Повторяется {parseRRuleToText(event.rrule)}
      </p>
    {/if}
  </HStack>

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

  <!-- Attendees -->
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

  <!-- Organizer -->
  {#if event.organiserName}
    <div class="bg-default ring-default rounded-xl p-2">
      <Body>
        Организатор: {event.organiserName}
      </Body>
    </div>
  {/if}

  <!-- Recurrence info -->
</HStack>
