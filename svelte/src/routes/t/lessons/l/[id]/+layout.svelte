<script lang="ts">
	import type { LayoutServerData } from './$types';

	let { data, children }: { data: LayoutServerData } = $props();
</script>

{@render children()}
