<script lang="ts">
	import { enhance } from '$app/forms';
	import { notification } from '$lib/stores';
	import { goto } from '$app/navigation';
	import { setUser } from '$lib/stores';

	const handleLoginResult = async ({ result, update }: { result: any; update: () => void }) => {

		if (result.type === 'success') {
			const { name, username, role } = result.data.user;
			const user = { name, username, role };
			localStorage.setItem('user', JSON.stringify(user));
			setUser(user);
			if (!user) {
				notification.set({ message: 'JWT Failure', type: 'error' });
				update();
				return;
			}

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

<div class="size-full items-center justify-center flex">
	<form
		method="POST"
		use:enhance={() => handleLoginResult}
		class="login-form bg-brick-50 p-8 rounded-md shadow-lg flex flex-col ring ring-milk-100  h-[500px]"
	>
		<h1 class="text-4xl">Welcome back</h1>

		<div class="my-4">
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
			class="bg-brick-700 ring-2 text-brick-50 px-4 py-2 w-20 mt-auto rounded-lg ring-milk-300 text-center hover:bg-brick-600 transition-colors"
			>Login</button
		>
	</form>
</div>
