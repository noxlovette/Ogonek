<script lang="ts">
	import Notification from '$lib/components/Notification.svelte';
	import type { LayoutData } from "./$types";
	import { setUser } from '$lib/stores';
	import '../app.css';

	import { user } from '$lib/stores';
	import { onMount } from 'svelte';

	export let data: LayoutData;
	onMount(() => {
    setUser(data.user);
  });
</script>

<div class="flex bg-sand-50 text-sand-900 h-screen w-screen items-center justify-center">
	<slot />
</div>

<Notification></Notification>
