<script lang="ts">
	let { children } = $props();
</script>

<div
	class="flex relative flex-col mx-4 md:mr-0 flex-1 px-5 md:px-6 lg:px-8 py-3 md:h-full h-[90%] scroll-my-6 lg:py-4 gap-4 overflow-y-auto rounded-lg ring-2 ring-milk-200 bg-white shadow-md"
>
	{@render children()}

</div>
