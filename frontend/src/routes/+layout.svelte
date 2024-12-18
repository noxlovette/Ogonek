<script lang="ts">
	import Loader from '$lib/components/Loader.svelte';
import Notification from '$lib/components/Notification.svelte';
	import '../app.css';
</script>

<main class="bg-sand-900/70 ">
<div class="flex flex-col md:flex-row text-sand-900 h-screen w-screen overflow-auto items-center justify-center max-w-7xl mx-auto selection:bg-forest-800 selection:text-sand-100">
	<slot />
</div>
<Notification></Notification>
<Loader></Loader>
</main>

