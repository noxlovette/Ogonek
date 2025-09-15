<script lang="ts">
  import { page } from "$app/state";
  import { goto } from "$app/navigation";
  import { formatEventDateTime, formatDuration } from "$lib/utils";
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

  const { data } = $props();
  const event = data.event;

  let isDeleting = $state(false);
  let isSharing = $state(false);

  async function deleteEvent() {
    if (!confirm(`Delete "${event.summary}"?`)) return;

    isDeleting = true;
    try {
      const response = await fetch(`/api/events/${event.uid}`, {
        method: "DELETE",
      });

      if (response.ok) {
        goto(`/${page.params.day}`);
      } else {
        alert("Failed to delete event");
      }
    } catch (error) {
      alert("Error deleting event");
    } finally {
      isDeleting = false;
    }
  }

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

  function parseRRuleToText(rrule?: string) {
    console.log("hello");
  }
</script>

<svelte:head>
  <title>{event.summary} | Calendar</title>
  <meta name="description" content={event.description || event.summary} />
</svelte:head>

<div class="min-h-screen bg-stone-50 dark:bg-stone-900">
  <!-- Header -->
  <header
    class="sticky top-0 z-10 border-b border-stone-200 bg-white/80 backdrop-blur-md dark:border-stone-800 dark:bg-stone-900/80"
  >
    <div class="mx-auto max-w-2xl px-4 py-3">
      <div class="flex items-center justify-between">
        <!-- Back button -->
        <button
          onclick={() => goto(`/${page.params.day}`)}
          class="flex items-center gap-2 text-stone-600 transition-colors hover:text-stone-900 dark:text-stone-400 dark:hover:text-stone-100"
        >
          <ChevronLeft class="h-5 w-5" />
          <span class="text-sm font-medium">Back</span>
        </button>

        <!-- Actions -->
        <div class="flex items-center gap-2">
          <button
            onclick={shareEvent}
            disabled={isSharing}
            class="hover:text-accent-600 dark:hover:text-accent-400 rounded-lg p-2 text-stone-600 transition-colors hover:bg-stone-100 dark:text-stone-400 dark:hover:bg-stone-800"
            title="Share event"
          >
            <Share class="h-5 w-5" />
          </button>

          <a
            href="/{page.params.day}/{event.uid}/edit"
            class="hover:text-accent-600 dark:hover:text-accent-400 rounded-lg p-2 text-stone-600 transition-colors hover:bg-stone-100 dark:text-stone-400 dark:hover:bg-stone-800"
            title="Edit event"
          >
            <Edit class="h-5 w-5" />
          </a>

          <button
            onclick={deleteEvent}
            disabled={isDeleting}
            class="rounded-lg p-2 text-stone-600 transition-colors hover:bg-stone-100 hover:text-red-600 dark:text-stone-400 dark:hover:bg-stone-800 dark:hover:text-red-400"
            title="Delete event"
          >
            <Trash2 class="h-5 w-5" />
          </button>
        </div>
      </div>
    </div>
  </header>

  <!-- Content -->
  <main class="mx-auto max-w-2xl px-4 py-6">
    <!-- Status banner for cancelled events -->
    {#if event.status === "cancelled"}
      <div
        class="mb-6 rounded-xl border border-red-200 bg-red-50 p-4 dark:border-red-800 dark:bg-red-900/20"
      >
        <p class="font-medium text-red-800 dark:text-red-200">
          This event has been cancelled
        </p>
      </div>
    {/if}

    <!-- Event header -->
    <div class="mb-8">
      <h1 class="mb-2 text-3xl font-bold text-stone-900 dark:text-stone-100">
        {event.summary}
      </h1>

      {#if event.categories?.length}
        <div class="mb-4 flex flex-wrap gap-2">
          {#each event.categories as category}
            <span
              class="bg-accent-100 dark:bg-accent-900/30 text-accent-800 dark:text-accent-200 inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium"
            >
              {category}
            </span>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Event details grid -->
    <div class="space-y-6">
      <!-- Date & Time -->
      <div
        class="rounded-2xl border border-stone-200 bg-white p-6 shadow-sm dark:border-stone-700 dark:bg-stone-800"
      >
        <div class="flex items-start gap-4">
          <div
            class="bg-accent-100 dark:bg-accent-900/30 flex-shrink-0 rounded-lg p-2"
          >
            <Clock class="text-accent-600 dark:text-accent-400 h-5 w-5" />
          </div>
          <div class="flex-1">
            <h2
              class="mb-2 text-lg font-semibold text-stone-900 dark:text-stone-100"
            >
              Date & Time
            </h2>
            <div class="space-y-1">
              <p class="text-stone-700 dark:text-stone-300">
                {formatEventDateTime(event.dtstart, event.dtend, event.allDay)}
              </p>
              {#if event.dtend && !event.allDay}
                <p class="text-sm text-stone-500 dark:text-stone-400">
                  Duration: {formatDuration(event.dtstart, event.dtend)}
                </p>
              {/if}
            </div>
          </div>
        </div>
      </div>

      <!-- Location -->
      {#if event.location}
        <div
          class="rounded-2xl border border-stone-200 bg-white p-6 shadow-sm dark:border-stone-700 dark:bg-stone-800"
        >
          <div class="flex items-start gap-4">
            <div
              class="bg-accent-100 dark:bg-accent-900/30 flex-shrink-0 rounded-lg p-2"
            >
              <MapPin class="text-accent-600 dark:text-accent-400 h-5 w-5" />
            </div>
            <div class="flex-1">
              <h2
                class="mb-2 text-lg font-semibold text-stone-900 dark:text-stone-100"
              >
                Location
              </h2>
              <p class="text-stone-700 dark:text-stone-300">{event.location}</p>
              <!-- Optional: Add map integration -->
              <a
                href="https://maps.google.com/?q={encodeURIComponent(
                  event.location,
                )}"
                target="_blank"
                rel="noopener noreferrer"
                class="text-accent-600 dark:text-accent-400 hover:text-accent-700 dark:hover:text-accent-300 mt-2 inline-flex items-center gap-1 text-sm"
              >
                Open in Maps
                <svg
                  class="h-3 w-3"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width={2}
                    d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
                  />
                </svg>
              </a>
            </div>
          </div>
        </div>
      {/if}

      <!-- Description -->
      {#if event.description}
        <div
          class="rounded-2xl border border-stone-200 bg-white p-6 shadow-sm dark:border-stone-700 dark:bg-stone-800"
        >
          <h2
            class="mb-3 text-lg font-semibold text-stone-900 dark:text-stone-100"
          >
            Description
          </h2>
          <div class="prose prose-stone dark:prose-invert max-w-none">
            <!-- Handle potential HTML in description -->
            <p class="whitespace-pre-wrap text-stone-700 dark:text-stone-300">
              {event.description}
            </p>
          </div>
        </div>
      {/if}

      <!-- Attendees -->
      {#if event.attendees?.length}
        <div
          class="rounded-2xl border border-stone-200 bg-white p-6 shadow-sm dark:border-stone-700 dark:bg-stone-800"
        >
          <div class="flex items-start gap-4">
            <div
              class="bg-accent-100 dark:bg-accent-900/30 flex-shrink-0 rounded-lg p-2"
            >
              <Users class="text-accent-600 dark:text-accent-400 h-5 w-5" />
            </div>
            <div class="flex-1">
              <h2
                class="mb-3 text-lg font-semibold text-stone-900 dark:text-stone-100"
              >
                Attendees ({event.attendees.length})
              </h2>
              <div class="space-y-2">
                {#each event.attendees as attendee}
                  <div class="flex items-center gap-3">
                    <!-- Avatar placeholder -->
                    <div
                      class="bg-accent-100 dark:bg-accent-900/30 flex h-8 w-8 items-center justify-center rounded-full"
                    >
                      <span
                        class="text-accent-600 dark:text-accent-400 text-xs font-medium"
                      >
                        {attendee.name?.charAt(0).toUpperCase()}
                      </span>
                    </div>
                    <span class="text-stone-700 dark:text-stone-300"
                      >{attendee.name}</span
                    >
                  </div>
                {/each}
              </div>
            </div>
          </div>
        </div>
      {/if}

      <!-- Organizer -->
      {#if event.organiserName}
        <div
          class="rounded-2xl border border-stone-200 bg-white p-6 shadow-sm dark:border-stone-700 dark:bg-stone-800"
        >
          <h2
            class="mb-2 text-lg font-semibold text-stone-900 dark:text-stone-100"
          >
            Organiser
          </h2>
          <p class="text-stone-700 dark:text-stone-300">
            {event.organiserName}
          </p>
        </div>
      {/if}

      <!-- Recurrence info -->
      {#if event.rrule}
        <div
          class="rounded-2xl border border-stone-200 bg-white p-6 shadow-sm dark:border-stone-700 dark:bg-stone-800"
        >
          <div class="flex items-start gap-4">
            <div
              class="bg-accent-100 dark:bg-accent-900/30 flex-shrink-0 rounded-lg p-2"
            >
              <Calendar class="text-accent-600 dark:text-accent-400 h-5 w-5" />
            </div>
            <div class="flex-1">
              <h2
                class="mb-2 text-lg font-semibold text-stone-900 dark:text-stone-100"
              >
                Repeats
              </h2>
              <p class="text-stone-700 dark:text-stone-300">
                <!-- You'll need a helper to parse RRULE into human readable text -->
                {parseRRuleToText(event.rrule)}
              </p>
            </div>
          </div>
        </div>
      {/if}
    </div>

    <!-- Action buttons -->
    <div class="mt-8 flex gap-3">
      <a
        href="/{page.params.day}/{event.uid}/edit"
        class="bg-accent-600 dark:bg-accent-500 hover:bg-accent-700 dark:hover:bg-accent-600 flex-1 rounded-xl px-6 py-3 text-center font-medium text-white transition-colors"
      >
        Edit Event
      </a>

      <button
        onclick={() => goto(`/${page.params.day}`)}
        class="rounded-xl bg-stone-200 px-6 py-3 font-medium text-stone-700 transition-colors hover:bg-stone-300 dark:bg-stone-700 dark:text-stone-300 dark:hover:bg-stone-600"
      >
        Done
      </button>
    </div>
  </main>
</div>
