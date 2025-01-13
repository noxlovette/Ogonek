<script lang="ts">
	import { Search } from 'lucide-svelte';
	let { search = $bindable(''), placeholder = 'Search lessons...' } = $props();
</script>

<div class="relative flex-1">
	<Search class="absolute left-3 top-1/2 -translate-y-1/2 text-milk-400 " />
	<input
		type="text"
		bind:value={search}
		{placeholder}
		class="w-full pl-10 pr-4 py-2 border rounded-full focus:ring-2 bg-brick-50 border-milk-200 focus:ring-brick-500 focus:border-transparent
        focus:placeholder:text-brick-400/70
        placeholder:text-milk-500
        "
	/>
</div>
