<script lang="ts">
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';
	import type { ActionData } from './$types';
	import { setProfile, setUser, user } from '$lib/stores';

	export let form: ActionData;

	let isSubmitting = false;
</script>

<form
	method="POST"
	use:enhance={() => {
		isSubmitting = true;

		return async ({ result }) => {
			isSubmitting = false;

			if (result.type === 'success') {
				setUser(result.data.user);
				setProfile(result.data.profile);
				localStorage.setItem('user', JSON.stringify(result.data.user));
				localStorage.setItem('profile', JSON.stringify(result.data.profile));
				goto($user.role === 'teacher' ? '/t/dashboard' : '/s/dashboard');
			}
		};
	}}
	class="space-y-4 max-w-md mx-auto p-6"
>
	<div class="space-y-2">
		<label for="username" class="block text-sm font-medium text-gray-700"> Username </label>
		<input
			id="username"
			name="username"
			type="text"
			class="w-full px-3 py-2 border rounded-md focus:ring-2 focus:ring-blue-500"
			required
			autocomplete="username"
		/>
	</div>

	<div class="space-y-2">
		<label for="password" class="block text-sm font-medium text-gray-700"> Password </label>
		<input
			id="password"
			name="password"
			type="password"
			class="w-full px-3 py-2 border rounded-md focus:ring-2 focus:ring-blue-500"
			required
			autocomplete="current-password"
		/>
	</div>

	{#if form?.message}
		<div class="text-red-500 text-sm" role="alert">
			{form.message}
		</div>
	{/if}

	<button
		type="submit"
		disabled={isSubmitting}
		class="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700
      disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
	>
		{#if isSubmitting}
			<span class="inline-block animate-spin mr-2">↻</span>
		{/if}
		{isSubmitting ? 'Signing in...' : 'Sign in'}
	</button>
</form>
