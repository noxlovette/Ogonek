<script lang="ts">
	import '../app.css';
	import { Header, Footer, Notification } from '$lib/components';
	interface Props {
		children?: import('svelte').Snippet;
	}
	let { children }: Props = $props();
</script>

<main class="bg-milk-50 overflow-auto min-h-screen antialiased w-screen h-screen">
	<div
		class="flex flex-col overscroll-contain font-medium items-center text-brick-800 max-w-7xl mx-auto selection:bg-pakistan-400 caret-brick-800 h-full"
	>
		<Header />
		<div class="flex flex-row size-full py-4">
			{@render children?.()}
		</div>
		<Footer />
	</div>
	<Notification></Notification>
</main>

<style lang="postcss">
	:global(.markdown h1) {
		@apply text-3xl font-bold opacity-80;
	}

	:global(.markdown h2) {
		@apply text-2xl font-bold opacity-70;
	}

	:global(.markdown h2) {
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

	:global(hr) {
		@apply border border-milk-200 w-1/2 my-2;
	}
</style>
