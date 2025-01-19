<script lang="ts">
	let { children } = $props();
</script>

<h2 class="font-medium text-balance text-lg/snug sm:text-xl/snug lg:text-2xl/snug">
	{@render children()}
</h2>
