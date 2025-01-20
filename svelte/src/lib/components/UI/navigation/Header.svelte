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

<header class="md:w-full ring-milk-200 ring-2 rounded-lg my-2 shadow-md items-baseline w-11/12">
	<div class="flex justify-between items-center w-full max-w-7xl mx-auto px-4 py-3">
		<div class="flex md:w-1/6 w-full">
			<a href="/" class="text-2xl font-serif font-bold">Firelight</a>
		</div>
		{#if $user.name}
			<Search />
			<div class="md:flex hidden space-x-4 items-center min-w-fit w-1/6">
				<p class="flex-shrink text-sm">
					{$user.name}, good {greeting}
				</p>
			</div>
		{:else}
			<div class="flex items-center space-x-2 md:space-x-4 md:w-1/6">
				<a href="/auth/login" class="md:text-lg text-sm font-serif font-bold">Login</a>
				<a href="/auth/signup" class="md:text-lg text-sm font-serif font-bold">Signup</a>
			</div>
		{/if}
	</div>
</header>
