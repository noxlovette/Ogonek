<script lang="ts">
	let { children } = $props();
</script>

<div
	class="flex flex-col flex-1 px-8 py-4 ml-4 gap-4 overflow-y-auto rounded-lg ring-2 ring-milk-200 bg-white shadow-md"
>
	{@render children()}
</div>
