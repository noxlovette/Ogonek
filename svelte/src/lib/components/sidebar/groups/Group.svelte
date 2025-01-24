<script lang="ts">
	let { children } = $props();
</script>

<div
	class="p-2 lg:p-3 xl:p-4 flex flex-col ring-2 dark:ring-milk-900 ring-milk-200 rounded-lg shadow-md"
>
	{@render children()}
</div>
