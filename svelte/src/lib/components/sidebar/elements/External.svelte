<script lang="ts">
	let { children, href = "/" } = $props();
</script>

<a
	href={href}
    target="_blank"
	rel="noopener noreferrer"
	class="lg:px-2 lg:py-4 px-1 py-2 inline-flex transition-all hover:translate-x-1"
>
	{@render children()}
</a>
