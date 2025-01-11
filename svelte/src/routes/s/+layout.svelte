<script lang="ts">
	import Sidebar from '$lib/components/chunks/Sidebar.svelte';
	import Rightbar from '$lib/components/chunks/Rightbar.svelte';
	import { setContext } from 'svelte';
	import BottomMenu from '$lib/components/mobile/BottomMenu.svelte';
	import type { Lesson } from '$lib/types';

	interface Props {
		data: any;
		children?: import('svelte').Snippet;
	}

	let { data, children }: Props = $props();

	// const tasks: App.Task[] = data.tasks;
	const lessons: Lesson[] = data.lessons;

	// setContext('tasks', tasks);
	setContext('lessons', lessons);
	// setContext('word', data.word);
</script>

<div class="flex flex-row size-full py-4">
	<Sidebar />
	<div class="flex flex-col justify-start items-center font-medium overflow-auto flex-1">
		<div class="flex flex-1 flex-col size-full px-8 py-4">
			{@render children?.()}
		</div>
		<BottomMenu />
	</div>
	<Rightbar />
</div>

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
		color: #737d3f;
		text-decoration: underline;
	}

	:global(.markdown ul, .markdown ol) {
		@apply my-2 ml-4;
	}

	:global(hr) {
		@apply border border-milk-200 w-1/2 my-2;
	}
</style>
