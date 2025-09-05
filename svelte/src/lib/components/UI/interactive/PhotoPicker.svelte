<script lang="ts">
  import { enhance } from "$app/forms";
  import { Check, ExternalLink, X } from "lucide-svelte";
  import type { Basic as Photo } from "unsplash-js/dist/methods/photos/types";
  import { UniButton } from "../forms";
  import { enhanceForm } from "$lib/utils";
  import { HStack } from "..";
  import { fade } from "svelte/transition";
  import { Caption1 } from "$lib/components/typography";

  let {
    photos,
    chosen = $bindable(),
  }: {
    photos?: Photo[];
    chosen?: string;
  } = $props();

  let selectedPhoto = $derived(
    chosen && photos ? photos.find((p) => p.id === chosen) : null,
  );

  function selectPhoto(photo: Photo) {
    chosen = photo.id;
  }

  function clearSelection() {
    chosen = undefined;
  }

  // Better keyboard navigation
  function handleKeydown(event: KeyboardEvent, photo: Photo) {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      selectPhoto(photo);
    }
  }
</script>

{#if photos}
  <div class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6" aria-label="Photo selection grid" in:fade>
    {#each photos as photo, index (photo.id)}
      <button
        type="button"
        onclick={() => selectPhoto(photo)}
        onkeydown={(e) => handleKeydown(e, photo)}
        class="group relative aspect-square w-full overflow-hidden rounded-2xl transition-all duration-200
               {chosen === photo.id
          ? 'ring-2 ring-amber-500/30 ring-offset-2 ring-offset-stone-50'
          : 'hover:ring-2 hover:ring-stone-300/50'}"
        role="gridcell"
        tabindex="0"
        aria-selected={chosen === photo.id}
        aria-label="{photo.alt_description || `Photo ${index + 1}`} by {photo
          .user.name}. {chosen === photo.id ? 'Selected' : 'Not selected'}"
      >
        <img
          src={photo.urls.small}
          alt="{photo.alt_description || `Photo ${index + 1}`} by {photo.user
            .name}"
          class="h-full w-full object-cover transition-transform duration-300
                 {chosen === photo.id ? 'scale-105' : 'group-hover:scale-105'}"
          loading="lazy"
        />
        {#if chosen === photo.id}
          <div
            class="absolute inset-0 bg-amber-500/20 backdrop-blur-[1px]"
            aria-hidden="true"
          >
            <div
              class="absolute top-2 right-2 rounded-full bg-amber-600 p-1.5 shadow-lg"
            >
              <Check class="text-white" size={16} />
            </div>
          </div>
        {/if}
        <div
          class="absolute right-0 bottom-0 left-0 bg-gradient-to-t from-stone-900/60 to-transparent p-3"
        ></div>
      </button>
    {/each}
  </div>

  <!-- Selected Photo Actions -->
  {#if selectedPhoto}
    <div
      class="rounded-xl border border-stone-200 bg-stone-50 p-4 dark:border-stone-700 dark:bg-stone-800/50"
    >
      <div class="flex items-start gap-4">
        <!-- Thumbnail -->
        <div class="flex-shrink-0">
          <img
            src={selectedPhoto.urls.thumb}
            alt="Selected photo"
            class="h-16 w-16 rounded-lg object-cover ring-1 ring-stone-200 dark:ring-stone-700"
          />
        </div>

        <!-- Info & Actions -->
        <div class="min-w-0 flex-1">
          <div class="mb-3">
            <h3 class="font-medium text-stone-900 dark:text-stone-100">
              {selectedPhoto.alt_description || "Untitled Photo"}
            </h3>
            <p class="text-sm text-stone-600 dark:text-stone-400">
              Photo by
              <span class="font-medium">{selectedPhoto.user.name}</span>
            </p>
          </div>

          <!-- Action buttons -->
          <div class="flex items-center gap-2">
            <button
              type="button"
              onclick={clearSelection}
              class="inline-flex items-center gap-1.5 rounded-lg px-3 py-1.5 text-sm
                       text-stone-600 transition-colors hover:bg-stone-100 hover:text-stone-800
                       dark:text-stone-400 dark:hover:bg-stone-700 dark:hover:text-stone-200"
              aria-label="Clear selection"
            >
              <X size={14} />
              Clear
            </button>

            <a
              href={selectedPhoto.links.html}
              target="_blank"
              rel="noopener noreferrer"
              class="inline-flex items-center gap-1.5 rounded-lg px-3 py-1.5 text-sm
                       text-stone-600 transition-colors hover:bg-stone-100 hover:text-stone-800
                       dark:text-stone-400 dark:hover:bg-stone-700 dark:hover:text-stone-200"
              aria-label="View on Unsplash (opens in new tab)"
            >
              <ExternalLink size={14} />
              Unsplash
            </a>

            <form
              use:enhance={enhanceForm({
                messages: {
                  success: "Photo selected successfully",
                },
              })}
              action="?/addPhoto"
              method="POST"
              class="ml-auto"
            >
              <input type="hidden" name="photoId" value={selectedPhoto.id} />
              <UniButton variant="prominent" type="submit" Icon={Check}>
                Confirm Selection
              </UniButton>
            </form>
          </div>
        </div>
      </div>
    </div>
  {/if}
{/if}
