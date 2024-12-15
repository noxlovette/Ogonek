<script lang="ts">
	import Sidebar from '$lib/components/Sidebar.svelte';
	import Rightbar from '$lib/components/Rightbar.svelte';


	import { setContext } from 'svelte';
	export let data: any;
	const tasks: App.Task[] = data.tasks;
	const lessons: App.Lesson[] = data.lessons;

	setContext('tasks', tasks);
	setContext('lessons', lessons);
</script>


	<Sidebar />
	<div class="flex flex-col w-full p-6 px-12 bg-sand-100 size-full border-l-2 border-r-2 border-sand-900/70 items-center justify-center">
		<slot />
	</div>
	<Rightbar />


	<style lang="postcss">
    

		:global(.markdown h1) {
			@apply text-2xl;
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
			color: #CC6D0E;
			text-decoration: underline;
		}
	
	
		:global(.markdown ul, .markdown ol) {
			@apply my-2 ml-4;
		}

		:global(hr) {
			@apply  border border-sand-900/20 w-1/2 my-2;
		}
	  </style>