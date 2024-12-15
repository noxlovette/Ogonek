<script lang="ts">
	import Sidebar from '$lib/components/Sidebar.svelte';
	import Rightbar from '$lib/components/Rightbar.svelte';


	import { setContext } from 'svelte';
	export let data: any;
	const tasks: App.Task[] = data.tasks;

	setContext('tasks', tasks);
</script>


	<Sidebar />
	<div class="flex flex-col w-full p-6 bg-sand-100 size-full border-l-2 border-r-2 border-sand-900/70 items-center justify-center">
		<slot />
	</div>
	<Rightbar />

