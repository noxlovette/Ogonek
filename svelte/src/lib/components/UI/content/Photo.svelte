<script lang="ts">
  import type { Photo, PhotoURLs } from "$lib/types";

  const { photo }: { photo: Photo | null | undefined } = $props();
</script>

{#if photo}
  <div class="relative h-30 w-full overflow-hidden rounded-t-xl">
    <div
      class="absolute inset-0 z-10 bg-cover bg-center"
      style="background-image: url('{(photo.urls as PhotoURLs).small}')"
    ></div>

    <div
      class="absolute inset-0 z-20 bg-cover bg-center"
      style="background-image: url('{(photo.urls as PhotoURLs).full}')"
    ></div>

    <img
      src={(photo.urls as PhotoURLs).full}
      alt={photo.altDescription}
      loading="lazy"
      class="absolute inset-0 -z-10 h-0 w-0 opacity-0"
    />

    {#if photo.photographerName}
      <div
        class="absolute right-0 bottom-0 z-30 m-2 rounded-full bg-stone-100 px-2 py-1 text-xs dark:bg-stone-800"
      >
        Photo by <a
          href="https://unsplash.com/@{photo.photographerUsername}?utm_source=Ogonek&utm_medium=referral"
          >{photo.photographerName}</a
        >
        on
        <a href="https://unsplash.com/?utm_source=Ogonek&utm_medium=referral"
          >Unsplash</a
        >
      </div>
    {/if}
  </div>
{/if}
