<script lang="ts">
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';
	import { ButtonSubmit } from '$lib/components/UI';
	import {
		setProfile,
		setUser,
		user,
		initialUser,
		initialProfile,
		notification
	} from '$lib/stores';
	import type { UserData } from '$lib/types';

	let isSubmitting = $state(false);
</script>

<div class="max-w-md space-y-8 bg-white p-8 rounded-xl shadow-md w-11/12">
	<div class="text-center">
		<h2 class="text-3xl font-bold text-brick-600">Welcome back</h2>
		<p class="mt-2 text-sm text-milk-600">
			Don't have an account?
			<a href="/auth/signup" class="font-medium text-brick-500 hover:text-brick-400">Sign up</a>
		</p>
	</div>
	<form
		method="POST"
		use:enhance={() => {
			isSubmitting = true;

			return async ({ result }) => {
				isSubmitting = false;

				if (result.type === 'success' && result.data) {
					const { user = initialUser, profile = initialProfile } = result.data;
					setUser(user);
					setProfile(profile);
					localStorage.setItem('user', JSON.stringify(user));
					localStorage.setItem('profile', JSON.stringify(profile));
					notification.set({ message: 'Welcome home', type: 'success' });
					await goto(user.role === 'teacher' ? '/t/dashboard' : '/s/dashboard');
				} else if (result.type === 'failure') {
					notification.set({
						message: String(result.data?.message) || "Something's off",
						type: 'error'
					});
				}
			};
		}}
		class="max-w-md space-y-6 mt-8"
	>
		<div class="space-y-4">
			<div>
				<label for="username" class="block text-sm font-medium text-milk-700"> Username </label>
				<input
					id="username"
					name="username"
					type="text"
					class="mt-1 block w-full px-3 py-2 bg-milk-50 border border-milk-300 rounded-md shadow-sm focus:outline-none focus:ring-brick-500 focus:border-brick-500"
					required
					autocomplete="username"
				/>
			</div>
		</div>
		<div class="space-y-2">
			<div>
				<label for="password" class="block text-sm font-medium text-milk-700"> Password </label>
				<input
					id="password"
					name="password"
					type="password"
					class="mt-1 block w-full px-3 py-2 bg-milk-50 border border-milk-300 rounded-md shadow-sm focus:outline-none focus:ring-brick-500 focus:border-brick-500"
					required
					autocomplete="current-password"
				/>
			</div>
		</div>

		<ButtonSubmit bind:isSubmitting buttonName="Login" />
	</form>
</div>
