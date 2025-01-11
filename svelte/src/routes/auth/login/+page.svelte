<script lang="ts">
	import { enhance } from '$app/forms';
	import { notification } from '$lib/stores';
	import { goto } from '$app/navigation';
	import { setUser } from '$lib/stores';

	const handleLoginResult = async ({ result, update }: { result: any; update: () => void }) => {
		console.log(result);
		if (result.type === 'success') {
			const { name, username } = result.data.user;
			const user = { name, username };
			setUser(user);
			if (!user) {
				notification.set({ message: 'JWT Failure', type: 'error' });
				update();
				return;
			}
			console.log(user);
			notification.set({ message: 'Welcome back!', type: 'success' });
			await goto('/s/dashboard');
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

<form
	method="POST"
	use:enhance={() => handleLoginResult}
	class="login-form bg-brick-50 p-8 rounded-md shadow-lg flex flex-col ring ring-milk-100 space-y-4"
>
	<h1 class="text-4xl">Welcome back</h1>

	<div>
		<p class="opacity-60 font-bold text-sm mb-1">Username</p>
		<input
			type="text"
			id="username"
			name="username"
			placeholder="Username"
			required
			class="rounded-lg bg-brick-50 p-2 ring-2 ring-milk-300"
		/>
	</div>

	<div>
		<p class="opacity-60 font-bold text-sm mb-1">Password</p>
		<input
			type="password"
			id="password"
			name="password"
			required
			placeholder="Password"
			class="rounded-lg bg-brick-50 p-2 ring-2 ring-milk-300"
		/>
	</div>

	<button
		type="submit"
		class="bg-brick-700 ring-2 text-brick-50 px-4 py-2 w-20 mt-5 rounded-lg ring-milk-300 text-center hover:bg-brick-600 transition-colors"
		>Login</button
	>
</form>
