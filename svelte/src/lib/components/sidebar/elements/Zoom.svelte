<script lang="ts">
	import { Video } from 'lucide-svelte';
	import External from './External.svelte';
	import { profile } from '$lib/stores';

	let href = $profile.zoomUrl ? $profile.zoomUrl : 'https://zoom.us';
</script>

<External {href}>
	<Video class="xl:size-8 lg:size-7 size-6 md:mr-2" />
	<p class="hidden md:flex">Zoom</p>
</External>
