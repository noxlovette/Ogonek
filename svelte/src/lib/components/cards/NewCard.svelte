<script lang="ts">
	import { PlusCircle } from 'lucide-svelte';
	import CardClickable from './CardClickable.svelte';

	let { href = '.', body = {} } = $props();
</script>

<form
	action={href}
	method="post"
	class="flex py-3 px-5 flex-col items-center justify-center hover:bg-cacao-600 hover:text-cacao-50 transition-colors shadow-md ring-2 ring-milk-200 rounded-lg max-h-[150px]"
>
	<button class="flex size-full items-center justify-center">
		<input type="hidden" name="body" value={body} />
		<PlusCircle class="size-12" />
	</button>
</form>
