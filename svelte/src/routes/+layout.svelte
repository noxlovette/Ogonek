<script lang="ts">
	import Loader from '$lib/components/Loader.svelte';
	import Notification from '$lib/components/Notification.svelte';
	import '../app.css';
	import { language } from '$lib/stores';
	import { onMount } from 'svelte';

	onMount(() => {
		language.set(localStorage.getItem('language') || 'en');
	});
</script>

<main class="bg-sand-900/70 overflow-auto min-h-screen antialiased w-screen">
	<div
		class="flex flex-col min-h-screen overscroll-contain md:flex-row text-sand-900 max-w-7xl w-full mx-auto selection:bg-forest-800 selection:text-sand-100"
	>
		<slot />
	</div>
	<Notification></Notification>
	<Loader></Loader>
</main>
