<script lang="ts">
	import { Loader2 } from 'lucide-svelte';
	let {
		isSubmitting = $bindable(false),
		onclick,
		formaction = undefined,
		type = undefined,
		styling = '',
		buttonName
	} = $props();
</script>

<button
	{type}
	{formaction}
	{onclick}
	disabled={isSubmitting}
	class={[
		styling,
		'px-4 py-2 flex justify-center items-center bg-brick-600 text-brick-50 rounded-lg hover:bg-brick-700 focus:outline-none focus:ring-2 focus:ring-brick-500 focus:ring-offset-2 transition-colors disabled:cursor-not-allowed disabled:opacity-50'
	]}
>
	{#if isSubmitting}
		<Loader2 class="size-5 animate-spin mr-1" />
	{/if}
	{isSubmitting ? 'On It...' : buttonName}
</button>
