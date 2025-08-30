<!-- PhotoPicker.svelte -->
<script lang="ts">
  import type { UnsplashPhoto, UnsplashResponse } from "$lib/types";

  let searchQuery = "";
  let photos: UnsplashPhoto[] = [];
  let loading = false;

  let response;
  $effect(() => {
    if (!searchQuery.trim()) {
      photos = [];
      return;
    }

    response = fetch(`/api/unsplash/search?query=${encodeURIComponent(query)}`);
  });

  function selectPhoto(photo) {
    selectedPhoto = photo;
    showPicker = false;
    console.log("chose");
  }

  function removePhoto() {
    selectedPhoto = null;
    console.log("removed");
  }
</script>

{#await response}{/await}
<div class="space-y-4">
  <div class="flex items-center justify-between">
    <label class="block text-sm font-medium text-stone-700">
      Lesson Photo
    </label>
    {#if selectedPhoto}
      <button
        on:click={removePhoto}
        class="text-sm text-red-600 hover:text-red-800"
      >
        Remove Photo
      </button>
    {/if}
  </div>

  {#if selectedPhoto}
    <div class="group relative">
      <img
        src={selectedPhoto.urls.regular}
        alt={selectedPhoto.alt_description || "Selected photo"}
        class="h-48 w-full rounded-lg object-cover"
      />
      <div
        class="bg-opacity-0 group-hover:bg-opacity-20 absolute inset-0 flex items-end rounded-lg bg-black p-4 transition-all"
      >
        <p
          class="text-sm text-white opacity-0 transition-opacity group-hover:opacity-100"
        >
          Photo by {selectedPhoto.user.name}
        </p>
      </div>
    </div>
  {:else}
    <button
      on:click={() => (showPicker = true)}
      class="group flex h-48 w-full flex-col items-center justify-center rounded-lg border-2 border-dashed border-stone-300 transition-colors hover:border-amber-600"
    >
      <svg
        class="h-12 w-12 text-stone-400 transition-colors group-hover:text-amber-600"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M12 6v6m0 0v6m0-6h6m-6 0H6"
        />
      </svg>
      <p class="mt-2 text-stone-500 group-hover:text-amber-700">
        Click to add a photo
      </p>
    </button>
  {/if}

  {#if showPicker}
    <div
      class="bg-opacity-50 fixed inset-0 z-50 flex items-center justify-center bg-black p-4"
    >
      <div
        class="max-h-[90vh] w-full max-w-4xl overflow-hidden rounded-xl bg-white"
      >
        <div class="border-b border-stone-200 p-6">
          <div class="mb-4 flex items-center justify-between">
            <h3 class="text-lg font-semibold text-stone-800">Choose a Photo</h3>
            <button
              on:click={() => (showPicker = false)}
              class="text-stone-400 hover:text-stone-600"
            >
              <svg
                class="h-6 w-6"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M6 18L18 6M6 6l12 12"
                />
              </svg>
            </button>
          </div>

          <input
            bind:value={searchQuery}
            placeholder="Search for photos..."
            class="w-full rounded-lg border border-stone-300 px-4 py-2 focus:border-transparent focus:ring-2 focus:ring-amber-500"
          />
        </div>

        <div class="max-h-[calc(90vh-150px)] overflow-y-auto p-6">
          {#if loading}
            <div class="flex h-32 items-center justify-center">
              <div
                class="h-8 w-8 animate-spin rounded-full border-b-2 border-amber-600"
              ></div>
            </div>
          {:else if photos.length > 0}
            <div class="grid grid-cols-2 gap-4 md:grid-cols-3 lg:grid-cols-4">
              {#each photos as photo (photo.id)}
                <button
                  on:click={() => selectPhoto(photo)}
                  class="group relative overflow-hidden rounded-lg transition-all hover:ring-2 hover:ring-amber-500"
                >
                  <img
                    src={photo.urls.small}
                    alt={photo.alt_description || "Photo"}
                    class="h-32 w-full object-cover transition-transform group-hover:scale-105"
                  />
                  <div
                    class="bg-opacity-0 group-hover:bg-opacity-20 absolute inset-0 bg-black transition-all"
                  ></div>
                </button>
              {/each}
            </div>
          {:else if searchQuery}
            <div class="py-12 text-center">
              <p class="text-stone-500">No photos found for "{searchQuery}"</p>
            </div>
          {:else}
            <div class="py-12 text-center">
              <p class="text-stone-500">Start typing to search for photos</p>
            </div>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</div>
