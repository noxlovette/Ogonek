<script lang="ts">
	let {
		isSubmitting = $bindable(false),
		formaction = undefined,
		type,
		styling = '',
		buttonName
	} = $props();
</script>

<button
	{type}
	{formaction}
	disabled={isSubmitting}
	class={[
		styling,
		'px-4 py-2 bg-brick-600 text-brick-50 rounded-lg hover:bg-brick-700 focus:outline-none focus:ring-2 focus:ring-brick-500 focus:ring-offset-2 disabled:opacity-50 transition-colors'
	]}
>
	{isSubmitting ? 'On It...' : buttonName}
</button>
