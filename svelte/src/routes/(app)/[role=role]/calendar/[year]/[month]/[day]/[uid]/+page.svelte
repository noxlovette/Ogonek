<script lang="ts">
  import { page } from "$app/state";
  import { goto } from "$app/navigation";
  import {
    formatEventDateTime,
    formatDuration,
    parseRRuleToText,
  } from "$lib/utils";
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

  let isDeleting = $state(false);
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
  <Title1 styling={event.status === "cancelled" ? "line-through" : ""}>
    {event.summary}
  </Title1>
  <!-- Date & Time -->
  <div class="bg-default ring-default rounded-xl p-2">
    <HStack>
      <p class="text-stone-700 dark:text-stone-300">
        {formatEventDateTime(event.dtstart, event.dtend, event.allDay)}
      </p>
      {#if event.dtend && !event.allDay}
        <p class="text-sm text-stone-500 dark:text-stone-400">
          Duration: {formatDuration(event.dtstart, event.dtend)}
        </p>
      {/if}
      {#if event.rrule}
        <p class="text-stone-700 dark:text-stone-300">
          <!-- You'll need a helper to parse RRULE into human readable text -->
          Повторяется {parseRRuleToText(event.rrule)}
        </p>
      {/if}
    </HStack>
  </div>

  <!-- Location -->
  {#if event.location}
    <div class="bg-default ring-default rounded-xl p-2">
      <Headline>
        {event.location}
      </Headline>

      <UniButton
        href={"https://yandex.com/maps/?text=" +
          encodeURIComponent(event.location)}
        Icon={MapPin}
        iconOnly={false}>Посмотреть на карте</UniButton
      >
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
              class="bg-accent/50 dark:bg-accent/30 flex size-8 items-center justify-center rounded-full"
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

  <VStack>
    <Merger>
      <UniButton Icon={Share} onclick={shareEvent} disable={isSharing} />
    </Merger>
    <Divider />
    <Merger>
      <DeleteButton confirmText="Delete Event" confirmTitle="Delete Event"
      ></DeleteButton>
      <EditButton href="{event.uid}/edit" />
    </Merger>
  </VStack>
</HStack>
