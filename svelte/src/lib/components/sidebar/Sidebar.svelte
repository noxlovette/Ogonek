<script lang="ts">
	let props = $props();
</script>

<div
	class={[
		props.class,
		'md:flex hidden h-max flex-shrink-0 bg-brick-600 dark:bg-brick-800/60 flex-col md:text-lg lg:text-xl xl:text-2xl font-serif w-1/5 px-4 py-2 ring-2 dark:ring-milk-900 ring-milk-200 rounded-lg shadow-md'
	]}
>
	<ul
		class={[
			props.subclass,
			'flex flex-col space-y-2 lg:space-y-3 xl:space-y-4 divide-y-2 divide-brick-400 dark:divide-milk-600'
		]}
	>
		{#each props.elements as Element}
			<Element />
		{/each}
	</ul>
</div>
