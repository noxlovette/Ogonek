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

<div class="max-w-md w-11/12 space-y-8 bg-white p-8 rounded-xl shadow-md">
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

		<ButtonSubmit bind:isSubmitting={loading} buttonName="Create Account"></ButtonSubmit>
	</form>
</div>
