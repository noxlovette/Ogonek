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
		'px-4 py-2 w-full flex justify-center items-center bg-brick-600 text-brick-50 rounded-lg hover:bg-brick-700 focus:outline-none focus:ring-2 focus:ring-brick-500 focus:ring-offset-2 transition-colors disabled:cursor-not-allowed disabled:opacity-50'
	]}
>
	{#if isSubmitting}
		<svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" fill="none" viewBox="0 0 24 24">
			<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
			<path
				class="opacity-75"
				fill="currentColor"
				d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
			/>
		</svg>
	{/if}
	{isSubmitting ? 'On It...' : buttonName}
</button>
