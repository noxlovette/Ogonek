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

<div
	class="max-w-md flex flex-col space-y-6 items-center justify-center dark:bg-milk-900 bg-white p-9 rounded-xl shadow-md w-11/12"
>
	<div class="text-center">
		<h2 class="text-3xl font-bold text-brick-600 dark:text-milk-200">Welcome back</h2>
		<p class="mt-2 text-sm text-milk-600">
			Don't have an account?
			<a
				href="/auth/signup"
				class="font-medium text-brick-500 dark:text-milk-50 dark:hover:text-milk-200 hover:text-brick-400"
				>Sign up</a
			>
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
		class="space-y-4 w items-center justify-center flex flex-col"
	>
		<div class="">
			<div>
				<label for="username" class="block text-sm font-medium text-milk-700"> Username </label>
				<input
					id="username"
					name="username"
					type="text"
					class="w-full px-4 py-2 border dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 dark:focus:outline-none dark:focus:ring-2 disabled:text-milk-500 border-milk-200 rounded-lg
            dark:bg-milk-950 focus:outline-none focus:ring-2 focus:ring-brick-500
                   transition duration-200"
					required
					autocomplete="username"
				/>
			</div>
		</div>
		<div class="">
			<div>
				<label for="password" class="block text-sm font-medium text-milk-700"> Password </label>
				<input
					id="password"
					name="password"
					type="password"
					class="w-full px-4 py-2 border dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 dark:focus:outline-none dark:focus:ring-2 disabled:text-milk-500 border-milk-200 rounded-lg
            dark:bg-milk-950 focus:outline-none focus:ring-2 focus:ring-brick-500
                   transition duration-200"
					required
					autocomplete="current-password"
				/>
			</div>
		</div>

		<div class="cf-turnstile my-4" data-sitekey="0x4AAAAAAA6Es9VtsFFGCAbw"></div>

		<ButtonSubmit bind:isSubmitting buttonName="Login" />
	</form>
</div>

<svelte:head>
	<title>Login</title>
	<script src="https://challenges.cloudflare.com/turnstile/v0/api.js" async defer></script>
</svelte:head>
