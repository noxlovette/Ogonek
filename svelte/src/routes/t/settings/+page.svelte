<script lang="ts">
	import { H1, SettingsPanel } from '$lib/components';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<H1>Settings</H1>
<SettingsPanel />
