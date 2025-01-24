<script lang="ts">
	import { Search } from 'lucide-svelte';
	import { isSearchOpen } from '$lib/stores';

	function handleKeydown(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
			e.preventDefault();
			isSearchOpen.set(true);
			searchElement.focus();
		}
		if (e.key === 'Escape') {
			isSearchOpen.set(false);
			query = '';
			searchElement.blur();
		}
	}

	function handleClick(e: MouseEvent) {
		if (searchElement && !searchElement.contains(e.target as Node)) {
			query = '';
			isSearchOpen.set(false);
			searchElement.blur();
		}
	}

	function handleSearchClick() {
		isSearchOpen.set(true);
		searchElement.focus();
	}
	let { query = $bindable(''), placeholder = 'Search...' } = $props();
	let searchElement: HTMLInputElement;

	const isMac = navigator.platform.toLowerCase().includes('mac');
	const cmdKey = isMac ? '⌘' : 'Ctrl';
</script>

<div class="relative flex-1">
	<Search class="absolute left-3 top-1/2 -translate-y-1/2 text-milk-400 " />
	<input
		type="text"
		bind:value={query}
		bind:this={searchElement}
		onclick={handleSearchClick}
		{placeholder}
		class="w-full pl-10 pr-4 py-2 border rounded-full focus:ring-2 bg-brick-50 dark:bg-milk-950 border-milk-200 dark:border-milk-900 dark:focus:ring-milk-700 dark:focus:placeholder:text-milk-700 focus:ring-brick-500 focus:border-transparent
        focus:placeholder:text-brick-400/70
        placeholder:text-milk-500
        "
	/>
	<div
		class="absolute right-3 top-1/2 -translate-y-1/2 md:flex hidden items-center gap-1 text-xs text-milk-400"
	>
		<kbd class="px-1.5 py-0.5 rounded bg-milk-100 border border-milk-300">{cmdKey}</kbd>
		<kbd class="px-1.5 py-0.5 rounded bg-milk-100 border border-milk-300">K</kbd>
	</div>
</div>

<svelte:window onkeydown={handleKeydown} onclick={handleClick} />
