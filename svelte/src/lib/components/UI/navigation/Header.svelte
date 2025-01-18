<script lang="ts">
	import { getGreeting } from '$lib/utils';
	import { user } from '$lib/stores';
	import Search from '../search/SearchHeader.svelte';
	const greeting = getGreeting();

	let dashboard = 's';
	if ($user.role === 'teacher') {
		dashboard = 't';
	}
</script>

<header class="w-full ring-milk-200 ring-2 rounded-lg my-2 shadow-md items-baseline">
	<div class="flex justify-between items-center w-full max-w-7xl mx-auto px-4 py-3">
		<div class="flex w-1/6">
			<a href="/" class="text-2xl font-serif font-bold">Firelight</a>
		</div>
		{#if $user.name}
			<Search />
			<div class="flex space-x-4 items-center min-w-fit w-1/6">
				<p class="flex-shrink text-sm">
					{$user.name}, good {greeting}
				</p>
			</div>
		{:else}
			<div class="flex items-center space-x-4 w-1/6">
				<a href="/auth/login" class="text-lg font-serif font-bold">Login</a>
				<a href="/auth/signup" class="text-lg font-serif font-bold">Signup</a>
			</div>
		{/if}
	</div>
</header>
