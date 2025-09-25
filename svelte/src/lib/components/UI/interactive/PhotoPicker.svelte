<script lang="ts">
  import { enhance } from "$app/forms";
  import { Ban, Check } from "lucide-svelte";
  import type { Basic as Photo } from "unsplash-js/dist/methods/photos/types";
  import { UniButton } from "../forms";
  import { enhanceForm } from "$lib/utils";
  import { HStack, Merger, VStack } from "..";
  import { fade } from "svelte/transition";
  import { Callout, Caption1, Title3 } from "$lib/components/typography";
  import Divider from "../toolbar/Divider.svelte";

  let {
    photos,
    chosen = $bindable(),
    error = undefined,
  }: {
    photos?: Photo[];
    chosen?: string;
    error?: boolean;
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

{#if error}
  <Callout>Unsplash error. Check back in later</Callout>
{/if}
{#if photos}
  <div
    class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6"
    aria-label="Photo selection grid"
    in:fade
  >
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
    <div class="ring-default bg-default flex w-full gap-4 rounded-2xl p-4">
      <!-- Thumbnail -->
      <div class="flex-shrink-0">
        <img
          src={selectedPhoto.urls.thumb}
          alt={selectedPhoto.alt_description}
          class="h-16 w-16 rounded-lg object-cover ring-1 ring-stone-200 dark:ring-stone-700"
        />
      </div>

      <HStack>
        <Title3 styling="capitalize text-clip">
          {selectedPhoto.alt_description || "Untitled Photo"}
        </Title3>
        <Caption1>
          Photo by
          <span class="font-medium">{selectedPhoto.user.name}</span>
        </Caption1>
      </HStack>
      <Divider></Divider>
      <VStack>
        <Merger>
          <UniButton content="Убрать выбор" Icon={Ban} onclick={clearSelection}
          ></UniButton>
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
            <UniButton
              content="Подтвердить выбор"
              variant="prominent"
              type="submit"
              Icon={Check}
            ></UniButton>
          </form>
        </Merger>
      </VStack>
    </div>
  {/if}
{/if}
