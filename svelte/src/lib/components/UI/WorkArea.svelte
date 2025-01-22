<script lang="ts">
	let { children } = $props();
</script>

<div
	class="flex relative flex-col mx-4 md:mr-0 flex-1 px-5 md:px-6 lg:px-8 py-3 md:h-full scroll-my-6 lg:py-4 gap-4 rounded-lg ring-2 ring-milk-200 dark:ring-milk-900 bg-white dark:bg-milk-950 shadow-md overflow-x-auto pb-24 md:pb-3"
>
	{@render children()}
</div>
