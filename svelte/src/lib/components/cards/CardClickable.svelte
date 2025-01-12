<script lang="ts">
	let { href, children } = $props();
</script>

<a
	{href}
	class="flex py-3 px-5 flex-col hover:bg-brick-600 hover:text-brick-50 transition-colors shadow-md ring-2 ring-milk-200 rounded-lg max-h-[150px] overflow-clip"
>
	{@render children()}
</a>
