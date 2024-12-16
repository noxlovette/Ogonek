<script lang="ts">
	import Notification from '$lib/components/Notification.svelte';
	import '../app.css';
</script>

<div class="bg-sand-900/70 ">
<div class="flex text-sand-900 h-screen w-screen items-center justify-center max-w-7xl mx-auto">
	<slot />
</div>
</div>

<Notification></Notification>
