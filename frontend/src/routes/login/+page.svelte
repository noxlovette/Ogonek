<script lang="ts">
	import { enhance } from '$app/forms';
	import { user, notification } from '$lib/stores';
	import { goto } from '$app/navigation';

	let username = '';
	let password = '';

	const handleLoginResult = async ({ result, update }) => {
		console.log('Received result:', result); // Log the entire result for debugging
		if (result.status === 302) {
			// Assuming 'location' is how the redirect URL is sent back
			notification.set({ message: 'Welcome back!', type: 'success' });
			await goto(result.location);
		} else {
			if (result.data) {
				notification.set({
					message: result.data.message || 'Login failed',
					type: 'error'
				});
			} else {
				notification.set({ message: 'Login failed', type: 'error' });
			}
		}
		update();
	};
</script>

<div
	class="absolute left-1/2 transform -translate-x-1/2 z-0 bg-roses-center bg-cover bg-center h-full opacity-50 w-full"
></div>
<div class="relative w-full max-w-[420px] my-auto mx-auto">
	<form
		method="POST"
		use:enhance={() => handleLoginResult}
		action="?/login"
		class="login-form bg-sand-900 p-3 rounded-md shadow-lg flex flex-col items-center ring ring-sand-900/20"
	>
		<h1 class="font-serif text-4xl text-center mb-4 text-sand-100">Firelight</h1>

		<input
			type="text"
			id="username"
			name="username"
			placeholder="Username"
			bind:value={username}
			required
			class="rounded-lg bg-white border border-sand-900/20 p-1 mb-2"
		/>

		<input
			type="password"
			id="password"
			name="password"
			bind:value={password}
			required
			placeholder="Password"
			class="rounded-lg bg-white border border-sand-900/20 p-1"
		/>

		<button
			type="submit"
			class="bg-sand-800/60 border-sand-800/80 border-2 text-sand-50 w-20 mt-5 rounded-lg p-2 text-center hover:bg-sand-200 hover:text-sand-800 transition-colors"
			>Login</button
		>
	</form>
</div>
