<script lang="ts">
	import { parseMarkdown } from '$lib/utils';

	let { markdownContent = $bindable('# Start writing\n\nYour **markdown** goes here...') } =
		$props();
	let htmlContent = $state('');
	let preview = $state(false);

	async function updatePreview(content: string) {
		htmlContent = await parseMarkdown(content);
	}

	$effect(() => {
		updatePreview(markdownContent);
	});
</script>

<div class="w-full h-full flex flex-col gap-4">
	<div id="header" class="flex space-x-4">
		<h1 class="text-2xl font-bold">Markdown</h1>
		<button
			onclick={() => (preview = false)}
			class="text-sm px-2 rounded-lg py-1 hover:bg-brick-200 transition-colors"
			class:chosen={!preview}>Editor</button
		>
		<button onclick={() => (preview = true)} 
            
            class="text-sm px-2 rounded-lg py-1 hover:bg-brick-200 transition-colors"
            
            
            class:chosen={preview}
			>Preview</button
		>
	</div>
	<div class="flex h-full">
		{#if !preview}
			<!-- Editor -->
			<div class="flex flex-col size-full">
				<textarea
					bind:value={markdownContent}
					class="flex-1 border-milk-200
                    resize-none focus:ring-transparent border
                    shadow-sm p-4
                    ring-0 ring-transparent rounded-lg focus:ring-none 
                    focus:border-milk-500"
					spellcheck="false"
				></textarea>
			</div>

			<!-- Preview -->
		{:else}
			<div class="flex flex-col">
				<div class="border p-4 rounded-lg overflow-y-auto markdown">
					{@html htmlContent}
				</div>
			</div>
		{/if}
	</div>
</div>

<style lang="postcss">
	.chosen {
		@apply bg-brick-600 text-milk-50 hover:bg-brick-600;
	}
</style>
