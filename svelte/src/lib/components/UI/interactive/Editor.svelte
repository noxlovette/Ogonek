<script lang="ts">
  import { parseMarkdown } from "$lib/utils";

  let {
    markdownContent = $bindable(
      "# Start writing\n\nYour **markdown** goes here...",
    ),
  } = $props();
  let htmlContent = $state("");
  let preview = $state(false);

  async function updatePreview(content: string) {
    htmlContent = await parseMarkdown(content);
  }

  $effect(() => {
    updatePreview(markdownContent);
  });
</script>

<div class="col-span-2 flex size-full flex-col gap-4">
  <!-- Added h-full here -->
  <div id="header" class="flex space-x-4">
    <h1 class="text-2xl font-bold">Markdown</h1>
    <button
      onclick={() => (preview = false)}
      class="hover:bg-cacao-200 rounded-lg px-2 py-1 text-sm transition-colors"
      class:chosen={!preview}>Editor</button
    >
    <button
      onclick={() => (preview = true)}
      class="hover:bg-cacao-200 rounded-lg px-2 py-1 text-sm transition-colors"
      class:chosen={preview}>Preview</button
    >
  </div>
  <div class="flex flex-1">
    <!-- Changed size-full to flex-1 -->
    {#if !preview}
      <!-- Editor -->
      <div class="flex w-full flex-col">
        <!-- Changed size-full to w-full -->
        <textarea
          bind:value={markdownContent}
          class="
          focus:border-cacao-500 focus:ring-cacao-500/20 min-h-[400px] w-full resize-none rounded-2xl border border-stone-300 bg-white px-4 py-2 text-base text-stone-900 placeholder-stone-400 shadow-sm transition-all focus:shadow-md focus:ring-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-60 dark:border-stone-700 dark:bg-stone-950 dark:text-stone-100
          "
          spellcheck="false"
        ></textarea>
      </div>
    {:else}
      <div
        class="markdown w-full rounded-lg p-4 shadow-sm dark:border-stone-900"
      >
        {@html htmlContent}
      </div>
    {/if}
  </div>
</div>
