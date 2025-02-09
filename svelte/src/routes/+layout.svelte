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
	class="bg-milk-50 dark:bg-milk-950 text-cacao-800 dark:text-milk-100 overflow-auto antialiased w-screen h-screen"
>
	<div
		class="flex flex-col font-medium items-center max-w-7xl mx-auto selection:bg-pakistan-700 selection:text-pakistan-50 dark:caret-milk-200 caret-cacao-800 h-full"
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
