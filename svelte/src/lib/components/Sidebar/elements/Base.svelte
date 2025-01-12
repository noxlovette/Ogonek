<script lang="ts">
	let { children, href = "/" } = $props();
</script>

<a
	href={href}
	class="lg:px-2 lg:py-4 px-1 py-2 inline-flex transition-all duration-300 hover:text-brick-100 hover:translate-x-1"
>
	{@render children()}
</a>
