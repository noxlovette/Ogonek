<script lang="ts">
	import { getGreeting } from '$lib/utils';
	import { user } from '$lib/stores';
	import Search from './SearchFull.svelte';
	const greeting = getGreeting();

	let dashboard = 's';
	if ($user.role === 'teacher') {
		dashboard = 't';
	}
</script>

<header class="w-full ring-milk-200 ring-2 rounded-lg my-2 shadow-md items-baseline">
	<div class="flex justify-between items-center w-full max-w-7xl mx-auto p-4">
		<div class="flex items-baseline space-x-4">
			<a href="/" class="text-2xl font-serif font-bold">Firelight</a>
			<nav class="flex items-center space-x-4">
				<a href="/{dashboard}/dashboard" class="text-lg font-serif font-bold">Dashboard</a>
				<a href="/t/docs" class="text-lg font-serif font-bold">Docs</a>
			</nav>
		</div>
		{#if $user.name}
			<div class="flex space-x-4 items-center">
				<p class="flex-shrink text-sm">
					Good {greeting}, {$user.name}
				</p>
				<Search />
			</div>
		{:else}
			<div class="flex items-center space-x-4">
				<a href="/auth/login" class="text-lg font-serif font-bold">Login</a>
				<a href="/auth/signup" class="text-lg font-serif font-bold">Signup</a>
			</div>
		{/if}
	</div>
</header>
