<script lang="ts">
	import type { LayoutServerData } from './$types';

	let { data, children }: { data: LayoutServerData } = $props();

	const { lesson } = data;
	
</script>

{@render children()}