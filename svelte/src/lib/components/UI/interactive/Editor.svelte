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

<div class="flex size-full flex-col gap-4">
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
          class="border-milk-200 dark:ring-milk-900 dark:bg-milk-950 dark:border-milk-900
					focus:ring-none
			  focus:border-milk-500 h-full min-h-[400px]
			  w-full resize-none rounded-lg border
			  p-4 shadow-sm ring-0 ring-transparent
			  focus:ring-transparent"
          spellcheck="false"
        ></textarea>
      </div>
    {:else}
      <div
        class="border-milk-200 dark:border-milk-900 markdown w-full rounded-lg border p-4"
      >
        {@html htmlContent}
      </div>
    {/if}
  </div>
</div>
