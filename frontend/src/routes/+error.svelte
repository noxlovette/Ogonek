<script lang="ts">
	import { page } from '$app/stores';
</script>

<div class="text-center flex flex-col rounded-lg items-center">
	
		<div class="flex size-full max-w-7xl flex-col items-center justify-center rounded-lg p-4 px-8 bg-sand-100">
			<h1 class="text-xl font-bold">Error {$page.status}: {$page.error.message}</h1>
			<a href="https://t.me/noxlovette" class="rounded text-center"
				>If you think you are not supposed to see this, please complain by clicking on this message.</a
			>
		</div>
        <a href="/u/dashboard" class="rounded-lg text-center text-xl font-bold p-2 bg-sand-100/20 hover:bg-sand-100/80 transition-colors mt-4 w-1/2" >Go back to dashboard</a>
	
</div>
