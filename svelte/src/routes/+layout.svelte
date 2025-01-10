<script lang="ts">
	import Loader from '$lib/components/UI/Loader.svelte';
	import Notification from '$lib/components/UI/Notification.svelte';
	import '../app.css';
	import { language } from '$lib/stores';
	import { onMount } from 'svelte';
	import Header from '$lib/components/UI/Header.svelte';
	import Footer from '$lib/components/UI/Footer.svelte';
	interface Props {
		children?: import('svelte').Snippet;
	}
	let { children }: Props = $props();
</script>

<main class="bg-sand-50 overflow-auto min-h-screen antialiased w-screen h-screen">
	<div
		class="flex flex-col overscroll-contain items-center text-sand-900 max-w-7xl mx-auto selection:bg-forest-800 selection:text-sand-100 h-full"
	>
		<Header />
		{@render children?.()}
		<Footer />
	</div>
	<Notification></Notification>
	<Loader></Loader>
</main>
