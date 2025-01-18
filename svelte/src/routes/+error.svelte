<script lang="ts">
	import { page } from '$app/stores';
</script>

<div class="text-center flex flex-col rounded-lg items-center w-full justify-center">
	<h1 class="text-xl font-bold">Error {$page.status}: {$page.error.message}</h1>
	<a href="https://t.me/noxlovette" class="rounded text-center"
		>If you think you are not supposed to see this, please complain by clicking on this message.</a
	>
	<a
		href="/"
		class="rounded-lg text-center text-xl font-bold px-4 py-2 bg-brick-400 hover:bg-brick-500 transition-colors mt-4"
		>Go back to dashboard</a
	>
</div>
