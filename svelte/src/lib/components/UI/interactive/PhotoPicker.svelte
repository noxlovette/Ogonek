<script lang="ts">
  import { enhance } from "$app/forms";
  import { EmptySpace } from "$lib/components/typography";
  import { Check, ExternalLink, X } from "lucide-svelte";
  import type { Basic as Photo } from "unsplash-js/dist/methods/photos/types";
  import { onMount } from "svelte";

  let { photos, chosen }: { photos?: Photo[]; chosen?: string } = $props();

  let selectedPhoto: Photo | null = $state(null);
  let gridElement: HTMLDivElement;
  let focusedIndex = $state(-1);
  let showActions = $state(false);
  let actionPanelElement: HTMLDivElement;

  $effect(() => {
    if (chosen && photos) {
      selectedPhoto = photos.find((p) => p.id === chosen) || null;
      showActions = !!selectedPhoto;
    } else {
      selectedPhoto = null;
      showActions = false;
    }
  });

  function selectPhoto(photo: Photo, index: number) {
    chosen = photo.id;
    selectedPhoto = photo;
    focusedIndex = index;
    showActions = true;
  }

  function deselectPhoto() {
    chosen = undefined;
    selectedPhoto = null;
    showActions = false;
    focusedIndex = -1;
  }

  function handleKeydown(event: KeyboardEvent, photo: Photo, index: number) {
    switch (event.key) {
      case "Enter":
      case " ":
        event.preventDefault();
        selectPhoto(photo, index);
        break;
      case "ArrowRight":
        event.preventDefault();
        if (photos && index < photos.length - 1) {
          const nextButton = gridElement.children[
            index + 1
          ] as HTMLButtonElement;
          nextButton?.focus();
          focusedIndex = index + 1;
        }
        break;
      case "ArrowLeft":
        event.preventDefault();
        if (index > 0) {
          const prevButton = gridElement.children[
            index - 1
          ] as HTMLButtonElement;
          prevButton?.focus();
          focusedIndex = index - 1;
        }
        break;
      case "ArrowDown":
        event.preventDefault();
        // Calculate next row (assuming 4 columns on lg screens)
        const cols =
          window.innerWidth >= 1024 ? 4 : window.innerWidth >= 768 ? 3 : 2;
        const nextRowIndex = index + cols;
        if (photos && nextRowIndex < photos.length) {
          const nextRowButton = gridElement.children[
            nextRowIndex
          ] as HTMLButtonElement;
          nextRowButton?.focus();
          focusedIndex = nextRowIndex;
        }
        break;
      case "ArrowUp":
        event.preventDefault();
        const prevCols =
          window.innerWidth >= 1024 ? 4 : window.innerWidth >= 768 ? 3 : 2;
        const prevRowIndex = index - prevCols;
        if (prevRowIndex >= 0) {
          const prevRowButton = gridElement.children[
            prevRowIndex
          ] as HTMLButtonElement;
          prevRowButton?.focus();
          focusedIndex = prevRowIndex;
        }
        break;
    }
  }

  function handleActionKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      showActions = false;
      // Return focus to selected photo
      if (focusedIndex >= 0) {
        const button = gridElement.children[focusedIndex] as HTMLButtonElement;
        button?.focus();
      }
    }
  }

  onMount(() => {
    if (actionPanelElement) {
      actionPanelElement.addEventListener("keydown", handleActionKeydown);
    }
  });
</script>

<div class="relative">
  {#if photos}
    <div
      bind:this={gridElement}
      class="grid grid-cols-2 gap-4 md:grid-cols-3 lg:grid-cols-4"
      role="grid"
      aria-label="Photo selection grid"
    >
      {#each photos as photo, index}
        <button
          type="button"
          onclick={() => selectPhoto(photo, index)}
          onkeydown={(e) => handleKeydown(e, photo, index)}
          class="group bg-default relative aspect-square cursor-pointer overflow-hidden
           rounded-2xl transition-all duration-200 focus:ring-2 focus:ring-blue-500 focus:outline-none
           {chosen === photo.id
            ? 'ring-3 ring-green-500'
            : 'hover:ring-accent hover:ring-3'}"
          role="gridcell"
          tabindex="0"
          aria-selected={chosen === photo.id}
          aria-label="{photo.alt_description || `Photo ${index + 1}`} by {photo
            .user.name}. {chosen === photo.id ? 'Selected' : 'Not selected'}"
        >
          <div
            class="absolute inset-0 animate-pulse bg-gray-200"
            aria-hidden="true"
          ></div>

          <img
            src={photo.urls.small}
            alt="{photo.alt_description || `Photo ${index + 1}`} by {photo.user
              .name}"
            class="h-full w-full object-cover transition-opacity duration-300"
            loading="lazy"
          />

          {#if chosen === photo.id}
            <div
              class="absolute inset-0 flex items-center justify-center bg-green-500/30"
              aria-hidden="true"
            >
              <Check class="text-green-500" size={32} />
            </div>
          {/if}
        </button>
      {/each}
    </div>

    <!-- Slide-out action panel -->
    <div
      bind:this={actionPanelElement}
      class="overflow-hidden transition-all duration-300 ease-in-out {showActions
        ? 'max-h-20'
        : 'max-h-0'}"
      aria-hidden={!showActions}
    >
      {#if selectedPhoto}
        <div class="mt-4 rounded-xl border border-gray-200 bg-gray-50 p-4">
          <div class="flex items-center justify-between gap-4">
            <div class="min-w-0 flex-1">
              <p class="truncate text-sm text-gray-600">
                Photo by <span class="font-medium"
                  >{selectedPhoto.user.name}</span
                >
              </p>
            </div>

            <div class="flex items-center gap-2">
              <button
                type="button"
                onclick={deselectPhoto}
                class="flex items-center gap-2 rounded-lg px-3 py-2 text-sm text-red-600 transition-colors hover:bg-red-50"
                aria-label="Deselect photo"
              >
                <X size={16} />
                Deselect
              </button>

              <a
                href={selectedPhoto.links.html}
                target="_blank"
                rel="noopener noreferrer"
                class="flex items-center gap-2 rounded-lg px-3 py-2 text-sm text-blue-600 transition-colors hover:bg-blue-50"
                aria-label="Visit photo on Unsplash (opens in new tab)"
              >
                <ExternalLink size={16} />
                Visit Unsplash
              </a>

              <form
                use:enhance
                action="?/addPhoto"
                method="POST"
                class="inline"
              >
                <input type="hidden" name="photoId" value={selectedPhoto.id} />
                <button
                  type="submit"
                  class="flex items-center gap-2 rounded-lg bg-green-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-green-700"
                  aria-label="Confirm photo selection"
                >
                  <Check size={16} />
                  Confirm
                </button>
              </form>
            </div>
          </div>
        </div>
      {/if}
    </div>
  {:else}
    <EmptySpace>Photos will show up here</EmptySpace>
  {/if}
</div>
