<script lang="ts">
	import Word from '$lib/components/Word.svelte';
	import Lessons from '../Lessons.svelte';
	import UsefulLInks from '../UsefulLInks.svelte';
	import { goto } from '$app/navigation';
	import { Menu } from 'lucide-svelte';
	import { user } from '$lib/stores';
	import { X } from 'lucide-svelte';
	export let isOpen = false;
	function toggleMenu() {
		isOpen = !isOpen;
	}

	function navigateAndCloseMenu(path) {
		goto(path); // Navigate to the new page
		isOpen = false; // Close the menu
	}
</script>

<div class="flex flex-col">
	<Word></Word>
	<Lessons></Lessons>
	<UsefulLInks></UsefulLInks>
</div>

<button
	class="fixed inset-0 z-40 bg-sand-900 bg-opacity-80 backdrop-blur-lg transition-all duration-300 ease-in-out md:hidden {isOpen
		? 'pointer-events-auto opacity-100'
		: 'pointer-events-none opacity-0'}"
	on:click|self={toggleMenu}
/>

<div class="flex md:hidden">
	<button
		on:click|stopPropagation={toggleMenu}
		class="rounded-lg bg-sand-300 px-4 py-2 transition-colors duration-300 hover:bg-gold-400 dark:bg-sand-800 dark:hover:text-sand-800"
	>
		<Menu class="size-8" />
	</button>

	<div
		class="absolute right-0 top-0 w-full transform space-y-4 rounded-lg
      bg-sand-300 px-8 py-12 shadow-lg
      
      transition-transform duration-300 ease-in-out dark:bg-sand-900 {isOpen
			? 'visible translate-x-0'
			: 'invisible translate-x-full'} z-50"
	>
		<div class="flex items-center justify-between">
			<h1 class="text-4xl">Firelight</h1>
			<button on:click={toggleMenu}>
				<X class="size-8 opacity-65" />
			</button>
		</div>
		<nav class="space-y-8">
			<ul class="flex flex-col space-y-2"></ul>
		</nav>
	</div>
</div>
