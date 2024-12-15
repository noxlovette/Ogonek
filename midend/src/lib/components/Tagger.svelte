<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { getContext } from 'svelte';

	const dispatch = createEventDispatcher();

	const article: App.Article = getContext('article');

	if (!article) {
		throw new Error('Article context not found');
	}
	let tags: String[] = [];

	if (article.tags_display) {
		tags = article.tags_display || [];
	}

	let newTag = '';

	function addTag() {
		if (newTag.trim() !== '') {
			tags = [...tags, newTag.trim()];
			newTag = '';
			dispatch('tagsUpdated', tags);
		}
	}

	function removeTag(index: number) {
		tags = tags.filter((_, i) => i !== index);
		dispatch('tagsUpdated', tags);
	}
</script>

{#each tags as tag}
	<input type="hidden" name="tags" value={tag} />
{/each}

<div class="space-y-2">
	<div class="flex items-center">
		<input
			type="text"
			placeholder="Add a tag"
			class="flex-1 px-3 py-2 border border-floral-300 bg-floral-50 rounded focus:outline-none focus:ring-2 focus:ring-tealdeuterium-500 focus:border-transparent"
			bind:value={newTag}
			on:keydown={(e) => {
				if (e.key === 'Enter' || e.key === ' ') {
					addTag();
				}
			}}
		/>
		<button
			type="button"
			class="ml-2 px-3 py-2 bg-tealdeuterium-500 text-white rounded-md hover:bg-tealdeuterium-600 focus:outline-none focus:ring-2 focus:ring-tealdeuterium-500 focus:ring-offset-2"
			on:click={addTag}
		>
			Add
		</button>
	</div>

	<div class="flex flex-wrap gap-2">
		{#each tags as tag, index}
			<div class="px-3 py-1 bg-floral-200 border rounded-md flex items-center justify-between">
				<span>{tag}</span>
				<button
					type="button"
					class="ml-2 text-dusk-500 hover:text-dusk-700 focus:outline-none"
					on:click={() => removeTag(index)}
				>
					&times;
				</button>
			</div>
		{/each}
	</div>
</div>
