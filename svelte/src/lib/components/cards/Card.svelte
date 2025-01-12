<script lang="ts">
	let { children } = $props();
</script>

<div
	class="flex py-3 px-5 flex-col shadow-md ring-2 ring-milk-200 rounded-lg max-h-[150px] overflow-clip"
>
	{@render children()}
</div>
