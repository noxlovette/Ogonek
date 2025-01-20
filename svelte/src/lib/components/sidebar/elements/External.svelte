<script lang="ts">
	let { children, href = '/' } = $props();
</script>

<a
	{href}
	target="_blank"
	rel="noopener noreferrer"
	class="lg:px-2 lg:py-4 md:px-1 md:py-2 inline-flex transition-all md:hover:translate-x-1"
>
	{@render children()}
</a>
