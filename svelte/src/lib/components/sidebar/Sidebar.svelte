<script lang="ts">
	let props = $props()
</script>

<div
	class={[props.class, "md:flex select-none bg-brick-600 text-brick-50 flex-col hidden md:text-lg lg:text-xl xl:text-2xl font-serif w-1/5 px-4 py-2 ring-2 ring-milk-200 rounded-lg shadow-md"]}
>
	<ul class="flex flex-col space-y-2 lg:space-y-3 xl:space-y-4">
		{#each props.elements as Element}
			<Element />
		{/each}
	</ul>
</div>
