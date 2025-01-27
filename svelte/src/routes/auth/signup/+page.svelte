<script lang="ts">
	import { enhance } from '$app/forms';
	import { notification } from '$lib/stores';
	import { goto } from '$app/navigation';
	import { Loader2 } from 'lucide-svelte';
	import ButtonSubmit from '$lib/components/UI/buttons/ButtonSubmit.svelte';

	let loading = $state(false);
	let password = $state('');
	let confirmPassword = $state('');
	let passwordMatch = $state(true);
	let isSubmitting = $state(false);
</script>

<div
	class="max-w-md flex flex-col space-y-6 items-center justify-center dark:bg-milk-900 bg-white p-9 rounded-xl shadow-md w-11/12"
>
	<div class="text-center">
		<h2 class="text-3xl font-bold text-brick-600">Create Account</h2>
		<p class="mt-2 text-sm text-milk-600">
			Already have an account?
			<a href="/auth/login" class="font-medium text-brick-500 hover:text-brick-400">Sign in</a>
		</p>
	</div>

	<form
		method="post"
		class="space-y-4 w items-center justify-center flex flex-col"
		use:enhance={() => {
			isSubmitting = true;

			return async ({ result }) => {
				isSubmitting = false;
				if (result.type === 'redirect') {
					notification.set({ message: 'Welcome on board', type: 'success' });
					goto(result.location);
				} else if (result.type === 'failure') {
					notification.set({
						message: String(result.data?.message) || "Something's off",
						type: 'error'
					});
				}
			};
		}}
	>
		<div class="space-y-4">
			<div>
				<label for="name" class="block text-sm font-medium text-milk-700">Full Name</label>
				<input
					type="text"
					name="name"
					required
					class="w-full px-4 py-2 border dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 dark:focus:outline-none dark:focus:ring-2 disabled:text-milk-500 border-milk-200 rounded-lg
            dark:bg-milk-950 focus:outline-none focus:ring-2 focus:ring-brick-500
                   transition duration-200"
				/>
			</div>

			<div>
				<label for="username" class="block text-sm font-medium text-milk-700">Username</label>
				<input
					type="text"
					name="username"
					required
					class="w-full px-4 py-2 border dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 dark:focus:outline-none dark:focus:ring-2 disabled:text-milk-500 border-milk-200 rounded-lg
            dark:bg-milk-950 focus:outline-none focus:ring-2 focus:ring-brick-500
                   transition duration-200"
				/>
			</div>

			<div>
				<label for="role" class="block text-sm font-medium text-milk-700">Role</label>
				<select
					name="role"
					required
					class="w-full px-4 py-2 border dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 dark:focus:outline-none dark:focus:ring-2 disabled:text-milk-500 border-milk-200 rounded-lg
            dark:bg-milk-950 focus:outline-none focus:ring-2 focus:ring-brick-500
                   transition duration-200"
				>
					<option value="">Select a role</option>
					<option value="teacher">Teacher</option>
					<option value="student">Student</option>
				</select>
			</div>

			<div>
				<label for="email" class="block text-sm font-medium text-milk-700">Email</label>
				<input
					type="email"
					name="email"
					required
					class="w-full px-4 py-2 border dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 dark:focus:outline-none dark:focus:ring-2 disabled:text-milk-500 border-milk-200 rounded-lg
            dark:bg-milk-950 focus:outline-none focus:ring-2 focus:ring-brick-500
                   transition duration-200"
				/>
			</div>

			<div>
				<label for="password" class="block text-sm font-medium text-milk-700">Password</label>
				<input
					type="password"
					name="password"
					bind:value={password}
					required
					class="w-full px-4 py-2 border dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 dark:focus:outline-none dark:focus:ring-2 disabled:text-milk-500 border-milk-200 rounded-lg
            dark:bg-milk-950 focus:outline-none focus:ring-2 focus:ring-brick-500
                   transition duration-200"
				/>
			</div>

			<div>
				<label for="confirmPassword" class="block text-sm font-medium text-milk-700"
					>Confirm Password</label
				>
				<input
					type="password"
					name="confirmPassword"
					bind:value={confirmPassword}
					required
					class="w-full px-4 py-2 border dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 dark:focus:outline-none dark:focus:ring-2 disabled:text-milk-500 border-milk-200 rounded-lg
            dark:bg-milk-950 focus:outline-none focus:ring-2 focus:ring-brick-500
                   transition duration-200"
				/>
				{#if !passwordMatch}
					<p class="mt-1 text-sm text-red-600">Passwords don't match</p>
				{/if}
			</div>
		</div>

		<ButtonSubmit bind:isSubmitting={loading} buttonName="Create Account"></ButtonSubmit>
	</form>
</div>

<svelte:head>
	<title>Signup</title>
</svelte:head>
