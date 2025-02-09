<script lang="ts">
	let { children, styling = '', gradient = true } = $props();
</script>

<div
	class={[
		styling,
		`relative flex flex-col p-5 rounded-xl bg-white ring-1 ring-milk-200
    shadow-lg shadow-milk-100
    hover:shadow-cacao-100
    hover:ring-cacao-400
    group transition-all duration-200 ease-out
    max-h-[150px] overflow-hidden`
	]}
>
	{@render children()}

	{#if gradient}
		<div
			class="absolute bottom-0 left-0 right-0 h-12
    bg-gradient-to-t from-white to-transparent
    pointer-events-none"
		></div>
	{/if}
</div>
