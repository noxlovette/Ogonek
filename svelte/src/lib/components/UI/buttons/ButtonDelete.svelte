<script lang="ts">
	import ButtonRaw from './ButtonRaw.svelte';

	let { isSubmitting = $bindable(false) } = $props();

	const type = 'submit';
	const buttonName = 'Delete';
	const formaction = '?/delete';
</script>

<ButtonRaw
	{type}
	{buttonName}
	{formaction}
	styling="bg-red-600 hover:bg-red-700"
	bind:isSubmitting
/>
