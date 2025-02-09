<script lang="ts">
	let { children, href = '/' } = $props();
</script>

<a {href} class="text-cacao-400 text-xs lg:text-sm font-sans p-1"> {@render children()} </a>
