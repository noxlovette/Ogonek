<script lang="ts">
	import Sidebar from '$lib/components/Sidebar.svelte';
	import Rightbar from '$lib/components/Rightbar.svelte';
	import { setUser } from '$lib/stores';
	import { onMount } from 'svelte';
	import { setContext } from 'svelte';
	import BottomMenu from '$lib/components/mobile/BottomMenu.svelte';

	export let data: any;
	onMount(() => {
		setUser(data.user);
	});

	const tasks: App.Task[] = data.tasks;
	const lessons: App.Lesson[] = data.lessons;

	setContext('tasks', tasks);
	setContext('lessons', lessons);
	setContext('word', data.word);
</script>

<Sidebar />

<div
	class="flex flex-col w-full py-12 px-6 lg:py-24 lg:px-12 bg-sand-100 size-full border-l-2 border-r-2 border-sand-900/70 items-center justify-center overflow-y-scroll custom-scrollbar"
>
	<slot />
</div>
<Rightbar />
<BottomMenu />

<style lang="postcss">
	:global(.markdown h1) {
		@apply text-2xl font-bold opacity-80;
	}

	:global(.markdown h2) {
		@apply text-xl font-bold opacity-60;
	}

	:global(.markdown p) {
		@apply m-1;
	}

	:global(.markdown ul) {
		list-style-type: disc;
	}

	:global(.markdown ol) {
		list-style-type: decimal;
	}

	:global(.markdown a) {
		color: #cc6d0e;
		text-decoration: underline;
	}

	:global(.markdown ul, .markdown ol) {
		@apply my-2 ml-4;
	}

	:global(hr) {
		@apply border border-sand-900/20 w-1/2 my-2;
	}
</style>
