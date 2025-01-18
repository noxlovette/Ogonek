<script lang="ts">
	import { enhance } from '$app/forms';
	import { notification } from '$lib/stores';
	import { goto } from '$app/navigation';

	let loading = $state(false);
	let password = $state('');
	let confirmPassword = $state('');
	let passwordMatch = $state(true);
	let isSubmitting = $state(false);
</script>

<div class="max-w-md w-full space-y-8 bg-white p-8 rounded-xl shadow-lg">
	<div class="text-center">
		<h2 class="text-3xl font-bold text-brick-600">Create Account</h2>
		<p class="mt-2 text-sm text-milk-600">
			Already have an account?
			<a href="/auth/login" class="font-medium text-brick-500 hover:text-brick-400">Sign in</a>
		</p>
	</div>

	<form
		method="post"
		class="mt-8 space-y-6"
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
					class="mt-1 block w-full px-3 py-2 bg-milk-50 border border-milk-300 rounded-md shadow-sm focus:outline-none focus:ring-brick-500 focus:border-brick-500"
				/>
			</div>

			<div>
				<label for="username" class="block text-sm font-medium text-milk-700">Username</label>
				<input
					type="text"
					name="username"
					required
					class="mt-1 block w-full px-3 py-2 bg-milk-50 border border-milk-300 rounded-md shadow-sm focus:outline-none focus:ring-brick-500 focus:border-brick-500"
				/>
			</div>

			<div>
				<label for="role" class="block text-sm font-medium text-milk-700">Role</label>
				<select
					name="role"
					required
					class="mt-1 block w-full px-3 py-2 bg-milk-50 border border-milk-300 rounded-md shadow-sm focus:outline-none focus:ring-brick-500 focus:border-brick-500"
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
					class="mt-1 block w-full px-3 py-2 bg-milk-50 border border-milk-300 rounded-md shadow-sm focus:outline-none focus:ring-brick-500 focus:border-brick-500"
				/>
			</div>

			<div>
				<label for="password" class="block text-sm font-medium text-milk-700">Password</label>
				<input
					type="password"
					name="password"
					bind:value={password}
					required
					class="mt-1 block w-full px-3 py-2 bg-milk-50 border border-milk-300 rounded-md shadow-sm focus:outline-none focus:ring-brick-500 focus:border-brick-500"
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
					class="mt-1 block w-full px-3 py-2 bg-milk-50 border border-milk-300 rounded-md shadow-sm focus:outline-none focus:ring-brick-500 focus:border-brick-500"
				/>
				{#if !passwordMatch}
					<p class="mt-1 text-sm text-red-600">Passwords don't match</p>
				{/if}
			</div>
		</div>

		<button
			type="submit"
			class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-brick-600 hover:bg-brick-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-brick-500 disabled:opacity-50 disabled:cursor-not-allowed"
			disabled={loading}
		>
			{#if loading}
				<svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" fill="none" viewBox="0 0 24 24">
					<circle
						class="opacity-25"
						cx="12"
						cy="12"
						r="10"
						stroke="currentColor"
						stroke-width="4"
					/>
					<path
						class="opacity-75"
						fill="currentColor"
						d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
					/>
				</svg>
			{/if}
			{loading ? 'Creating Account...' : 'Create Account'}
		</button>
	</form>
</div>
