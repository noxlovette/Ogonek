<script lang="ts">
	import { enhance } from '$app/forms';
	import { user, notification } from '$lib/stores';
	import { goto } from '$app/navigation';

	let username = '';
	let password = '';

	const handleLoginResult = async ({ result, update }) => {
		if (result.data.result.success) {
			$user.is_authenticated = true;
			$user.quizlet_url = result.data.result.quizlet_url;
			$user.username = result.data.result.username;
			notification.set({ message: 'Welcome back!', type: 'success' });
			await goto('/u/dashboard');
		} else {
			notification.set({
				message: result.data.error || 'Login failed',
				type: 'error'
			});
		}

		update();
	};
</script>

<div
	class="absolute left-1/2 transform -translate-x-1/2 z-0 bg-roses-center bg-cover bg-center w-1/2 h-1/2 opacity-50"
></div>
<div class="relative w-full max-w-[420px] mx-auto">
	<form
		method="POST"
		use:enhance={() => handleLoginResult}
		action="?/login"
		class="login-form bg-sand-50 p-3 rounded-md shadow-lg z-10 flex flex-col items-center ring ring-sand-800/60"
	>
		<h1 class="font-serif text-4xl text-center mb-4">Firelight</h1>

	
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
		
		<button type="submit" class="py-1 px-4 bg-sand-800/60 text-sand-50 hover:bg-forest-700 transition-colors rounded-lg mt-4 mb-4"
			>Login</button
		>
	</form>
</div>
