<script lang="ts">
	import '../app.css';
	import { Header, Footer, Notification } from '$lib/components';
	import SearchOverlay from '$lib/components/UI/search/SearchOverlay.svelte';
	interface Props {
		children?: import('svelte').Snippet;
	}
	let { children }: Props = $props();
</script>

<main
	class="bg-milk-50 dark:bg-milk-950 text-brick-800 dark:text-brick-100 overflow-auto antialiased w-screen h-screen"
>
	<div
		class="flex flex-col font-medium items-center max-w-7xl mx-auto selection:bg-pakistan-400 caret-brick-800 h-full"
	>
		<Header />
		<div id="main" class="flex flex-row w-full py-4 flex-1">
			{@render children?.()}
		</div>
		<Footer />
	</div>
	<Notification />
	<SearchOverlay />
</main>

<style lang="postcss">
	main {
		-webkit-overflow-scrolling: touch;
	}

	:global(.markdown h1) {
		@apply text-3xl font-bold opacity-80;
	}
	:global(.markdown h2) {
		@apply text-2xl font-bold opacity-70;
	}
	:global(.markdown h3) {
		@apply text-xl font-bold opacity-60;
	}
	:global(.markdown p) {
		@apply m-1;
	}
	:global(.markdown ul) {
		list-style-type: disc;
	}
	:global(.markdown ol) {
		list-style-type: decimal;
	}
	:global(.markdown a) {
		@apply text-pakistan-700 underline;
	}
	:global(.markdown ul, .markdown ol) {
		@apply my-2 ml-4;
	}
	:global(.markdown blockquote) {
		@apply border-l-4 border-milk-300 pl-4 italic opacity-90 my-4;
	}
	:global(.markdown pre) {
		@apply bg-milk-100 p-4 rounded-md overflow-x-auto my-4;
	}
	:global(.markdown code) {
		@apply bg-milk-50 text-sm font-mono px-1 rounded;
	}
	:global(.markdown hr) {
		@apply border border-milk-200 w-1/2 my-2;
	}
	:global(.markdown table) {
		@apply border-collapse border border-milk-300 w-full my-4;
	}
	:global(.markdown th) {
		@apply bg-milk-200 font-bold text-center border border-milk-300 px-2 py-1;
	}
	:global(.markdown td) {
		@apply border border-milk-300 px-2 py-1 text-left;
	}
</style>
