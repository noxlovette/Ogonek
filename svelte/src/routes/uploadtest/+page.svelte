<script lang="ts">
	import Uploader from '$lib/components/UI/Uploader.svelte';
	import { notification } from '$lib/stores';
</script>

<button onclick={() => notification.set({ message: 'Hello, world!', type: 'success' })}>
	Show Success
</button>
<button onclick={() => notification.set({ message: 'Hello, world!', type: 'error' })}>
	Show Error
</button>
<button onclick={() => notification.set({ message: 'Hello, world!', type: 'info' })}>
	Show Info
</button>
<Uploader />
