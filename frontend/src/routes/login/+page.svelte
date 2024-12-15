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
			await goto('/u/');
		} else {
			notification.set({
				message: result.data.error || 'Login failed',
				type: 'error'
			});
		}

		update();
	};
</script>

<form method="POST" use:enhance={() => handleLoginResult} action="?/login">
	<div>
		<label for="username">Username:</label>
		<input type="text" id="username" name="username" bind:value={username} required />
	</div>
	<div>
		<label for="password">Password:</label>
		<input type="password" id="password" name="password" bind:value={password} required />
	</div>
	<button type="submit">Login</button>
</form>
