<script lang="ts">
	interface Props {
		children?: import('svelte').Snippet;
	}

	let { children }: Props = $props();
</script>

<h1 class="text-4xl font-serif font-bold">
	{@render children?.()}
</h1>
