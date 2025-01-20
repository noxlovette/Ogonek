<script lang="ts">
	import ButtonRaw from './ButtonRaw.svelte';

	let { isSubmitting = $bindable(false), buttonName = 'Save', formaction = undefined, styling="", disabled=false } = $props();
	const type = 'submit';
</script>

<ButtonRaw {type} {buttonName} {formaction} bind:isSubmitting {styling} {disabled} />
