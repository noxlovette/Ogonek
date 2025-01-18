<script lang="ts">
	import { Search } from 'lucide-svelte';

	function handleKeydown(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
			e.preventDefault();
			searchElement.focus();
		}
		if (e.key === 'Escape') {
			searchElement.blur();
		}
		if (e.key === 'Enter') {
			console.log('Search submitted:', query);
		}
	}

	function handleClickOutside(e: MouseEvent) {
		if (searchElement && !searchElement.contains(e.target as Node)) {
			searchElement.blur();
		}
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
		{placeholder}
		class="w-full pl-10 pr-4 py-2 border rounded-full focus:ring-2 bg-brick-50 border-milk-200 focus:ring-brick-500 focus:border-transparent
        focus:placeholder:text-brick-400/70
        placeholder:text-milk-500
        "
	/>
	<div
		class="absolute right-3 top-1/2 -translate-y-1/2 flex items-center gap-1 text-xs text-milk-400"
	>
		<kbd class="px-1.5 py-0.5 rounded bg-milk-100 border border-milk-300">{cmdKey}</kbd>
		<kbd class="px-1.5 py-0.5 rounded bg-milk-100 border border-milk-300">K</kbd>
	</div>
</div>

<svelte:window onkeydown={handleKeydown} onclick={handleClickOutside} />
