<script lang="ts">
	import { applyAction, enhance } from '$app/forms';
	import type { Profile, User } from '$lib/types';
	import {
		user,
		profile,
		notification,
		setProfile,
		setUser,
		initialProfile,
		initialUser
	} from '$lib/stores';
	import ButtonSubmit from '../buttons/ButtonSubmit.svelte';

	let isSubmitting = $state(false);
</script>

<!-- User Settings -->
<form
	class="space-y-4 rounded-lg flex flex-col"
	method="POST"
	use:enhance={({ formData, cancel }) => {
		if (!formData) {
			cancel();
		}

		isSubmitting = true;

		return async ({ result }) => {
			isSubmitting = false;
			console.log(result);

			if (result.type === 'success' && result.data) {
				const { user = initialUser, profile = initialProfile } = result.data;
				setUser(user);
				setProfile(profile);
				localStorage.setItem('user', JSON.stringify(user));
				localStorage.setItem('profile', JSON.stringify(profile));
				notification.set({ message: 'Changes saved', type: 'success' });
			} else if (result.type === 'failure') {
				notification.set({
					message: String(result.data?.message) || "Something's off",
					type: 'error'
				});
			}
		};
	}}
	action="?/update"
>
	<div class="flex bg-milk-50 p-4 rounded-lg space-x-4">
		<div class="space-y-2">
			<h2 class="text-2xl font-semibold text-brick-800 mb-4">User Settings</h2>
			<div class="space-y-2">
				<label for="email" class="block text-sm font-medium text-brick-700"> Email </label>
				<input
					type="email"
					id="email"
					name="email"
					value={$user.email}
					class="w-full px-3 py-2 border border-milk-200 rounded-md focus:outline-none focus:ring-2 focus:ring-brick-500"
				/>
			</div>

			<div class="space-y-2">
				<label for="username" class="block text-sm font-medium text-brick-700"> Username </label>
				<input
					type="text"
					id="username"
					name="username"
					value={$user.username}
					class="w-full px-3 py-2 border border-milk-200 rounded-md focus:outline-none focus:ring-2 focus:ring-brick-500"
				/>
			</div>

			<div class="space-y-2">
				<label for="name" class="block text-sm font-medium text-brick-700"> Name </label>
				<input
					type="text"
					id="name"
					name="name"
					value={$user.name}
					class="w-full px-3 py-2 border border-milk-200 rounded-md focus:outline-none focus:ring-2 focus:ring-brick-500"
				/>
			</div>
		</div>
		<div class="space-y-2">
			<h2 class="text-2xl font-semibold text-brick-800 mb-4">Profile Settings</h2>
			<div class="space-y-2">
				<label for="quizlet" class="block text-sm font-medium text-brick-700"> Quizlet URL </label>
				<input
					type="url"
					id="quizlet"
					name="quizlet"
					value={$profile.quizletUrl}
					class="w-full px-3 py-2 border border-milk-200 rounded-md focus:outline-none focus:ring-2 focus:ring-brick-500"
				/>
			</div>

			<div class="space-y-2">
				<label for="zoom" class="block text-sm font-medium text-brick-700"> Zoom URL </label>
				<input
					type="url"
					id="zoom"
					name="zoom"
					value={$profile.zoomUrl}
					class="w-full px-3 py-2 border border-milk-200 rounded-md focus:outline-none focus:ring-2 focus:ring-brick-500"
				/>
			</div>
		</div>
	</div>
	<div>
		<ButtonSubmit bind:isSubmitting buttonName="Update Profile" />
	</div>
</form>
