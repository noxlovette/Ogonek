<script lang="ts">
	import Header from '$lib/components/typography/Header.svelte';
	import SettingsPanel from '$lib/components/UI/SettingsPanel.svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<Header>Settings</Header>
<SettingsPanel />
