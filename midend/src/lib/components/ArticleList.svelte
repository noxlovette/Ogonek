<script lang="ts">
	import { ArrowBigLeft, SquareArrowLeft } from 'lucide-svelte';
	import { getContext } from 'svelte';

	const meta: App.Meta = getContext('meta');
	const articles: App.Article[] = meta.articles;

	let searchTerm = '';
	let displayedArticles = articles;

	$: displayedArticles = articles.filter((article) => {
		const title = article.title || '';
		const content = article.content || '';
		return (
			title.toLowerCase().includes(searchTerm.toLowerCase()) ||
			content.toLowerCase().includes(searchTerm.toLowerCase())
		);
	});
</script>

<input
	type="text"
	placeholder="Search"
	bind:value={searchTerm}
	class="w-full px-3 py-2 border border-dusk-300 rounded-md focus:outline-none focus:ring-2 focus:ring-tealdeuterium-500 focus:border-transparent"
/>
<ul class="py-4 text-2xl font-bold">
	{#each displayedArticles as article}
		<li>
			<a href="/editor/{article.id}" class="hover:text-orangedeuterium-600" data-sveltekit-reload>
				{article.title}
			</a>
		</li>
	{/each}
</ul>
<a href="/editor/" class="mt-4 text-xl font-normal hover:text-orangedeuterium-700">
	<SquareArrowLeft class="inline-block w-8 h-8 align-bottom" />
	Back
</a>
