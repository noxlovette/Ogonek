<script lang="ts">
  import { enhance } from "$app/forms";
  import { EmptySpace } from "$lib/components/typography";
  import { Check } from "lucide-svelte";
  import type { Basic as Photo } from "unsplash-js/dist/methods/photos/types";

  let { photos, chosen }: { photos?: Photo[]; chosen?: string } = $props();
</script>

{#if photos}
  <form
    class="grid grid-cols-2 gap-4 md:grid-cols-3 lg:grid-cols-4"
    use:enhance
    action="?/addPhoto"
    method="POST"
  >
    <input type="hidden" name="photoId" value={chosen} />

    {#each photos as photo, index}
      <button
        onclick={() => (chosen = photo.id)}
        type="submit"
        class="group bg-default relative aspect-square cursor-pointer overflow-hidden
         rounded-2xl
         {chosen === photo.id
          ? 'ring-3 ring-green-500'
          : 'hover:ring-accent hover:ring-3'}"
      >
        <img
          src={photo.urls.small}
          alt={photo.alt_description || `Photo ${index + 1}`}
          class="h-full w-full object-cover"
          loading="lazy"
        />

        {#if chosen === photo.id}
          <div
            class="absolute inset-0 flex items-center justify-center bg-green-500/30"
          >
            <Check class="text-green-500" />
          </div>
        {/if}
      </button>
    {/each}
  </form>
{:else}
  <EmptySpace>Photos will show up here</EmptySpace>
{/if}
