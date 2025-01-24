<script lang="ts">
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';
	import {
		user,
		profile,
		notification,
		setProfile,
		setUser,
		initialProfile,
		initialUser,
		clearUser
	} from '$lib/stores';
	import ButtonSubmit from '../buttons/ButtonSubmit.svelte';
	import ButtonRaw from '../buttons/ButtonRaw.svelte';
	import { H2 } from '$lib/components/typography';

	let isSubmitting = $state(false);
	let disabled = $state(true);
</script>

<div class="grid md:grid-cols-2 grid-cols-1 gap-4">
	<form
		class="container col-span-2"
		method="POST"
		use:enhance={({ formData, cancel }) => {
			if (!formData) cancel();

			isSubmitting = true;

			return async ({ result }) => {
				isSubmitting = false;

				// Destructure with nullish coalescing for safer fallbacks
				const { user = initialUser, profile = initialProfile } = result.data ?? {};

				if (result.type === 'success' && result.data) {
					setUser(user);
					setProfile(profile);
					localStorage.setItem('user', JSON.stringify(user));
					localStorage.setItem('profile', JSON.stringify(profile));
					notification.set({ message: 'Changes saved ✨', type: 'success' });
					disabled = true;
				} else {
					notification.set({
						message: result.data?.message ?? "Yikes! Something's not right",
						type: 'error'
					});
				}
			};
		}}
		action="?/update"
	>
		<div
			class="grid grid-cols-1 md:grid-cols-2 gap-6 bg-milk-50 dark:bg-milk-900 p-6 rounded-xl shadow"
		>
			<!-- User Settings Section -->
			<div class="space-y-4">
				<h2 class="text-2xl font-bold text-brick-800 dark:text-milk-100">User Settings</h2>

				{#each ['email', 'username', 'name'] as field}
					<div class="space-y-2">
						<label
							for={field}
							class="block text-sm font-medium text-brick-700 dark:text-milk-100 capitalize"
						>
							{field}
						</label>
						<input
							type={field === 'email' ? 'email' : 'text'}
							id={field}
							{disabled}
							name={field}
							value={$user[field]}
							class="w-full px-4 py-2 border disabled:text-milk-500 border-milk-200 rounded-lg
            dark:bg-milk-950 focus:outline-none focus:ring-2 focus:ring-brick-500
                   transition duration-200"
						/>
					</div>
				{/each}
			</div>

			<!-- Profile Settings Section -->
			<div class="space-y-4">
				<h2 class="text-2xl font-bold text-brick-800 dark:text-milk-100">Profile Settings</h2>

				{#each ['quizlet', 'zoom'] as field}
					<div class="space-y-2">
						<label
							for={field}
							class="block text-sm font-medium text-brick-700 dark:text-milk-100 capitalize"
						>
							{field} URL
						</label>
						<input
							type="url"
							{disabled}
							id={field}
							name={field}
							value={$profile[`${field}Url`]}
							class="w-full px-4 py-2 border disabled:text-milk-500 border-milk-200 rounded-lg
                   focus:outline-none focus:ring-2 focus:ring-brick-500
                   transition duration-200 dark:bg-milk-950"
						/>
					</div>
				{/each}
			</div>

			<!-- Update Button - Spans full width -->
			<div class="md:col-span-2 mt-6 flex flex-row space-x-2">
				<ButtonSubmit
					bind:isSubmitting
					buttonName="Save Changes"
					{disabled}
					styling="w-full md:w-auto md:float-right"
				/>
				<button
					type="button"
					onclick={() => {
						disabled = !disabled;
					}}
					class="bg-milk-200 dark:bg-milk-800 text-sm md:text-base transition-colors hover:bg-milk-300 px-3 rounded-lg md:px-4 py-2"
				>
					{disabled ? 'Unlock' : 'Lock'}
				</button>
			</div>
		</div>
	</form>

	<section
		class="flex flex-col space-y-2 bg-milk-50 dark:bg-milk-900 rounded-lg shadow p-4 justify-between"
	>
		<H2>Telegram</H2>
		<p class="align-top">
			Click the button below and type in "/start" to the bot if you want to receive notifications
			about new tasks
		</p>
		<a
			href="https://t.me/fz_notif_bot"
			class="bg-cyan-600 hover:bg-cyan-700 transition-colors max-w-fit flex rounded-lg text-cyan-50 py-2 px-4"
		>
			Receive notifications
		</a>
	</section>

	<form
		action="?/logout"
		method="POST"
		class="md:flex hidden"
		use:enhance={() => {
			isSubmitting = true;

			return async ({ result }) => {
				isSubmitting = false;
				if (result.type === 'redirect') {
					clearUser();
					localStorage.removeItem('user');
					localStorage.removeItem('profile');
					notification.set({ message: 'Bye!', type: 'success' });
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
		<section class="flex flex-col bg-milk-50 dark:bg-milk-900 rounded-lg shadow p-4">
			<div class="flex-none">
				<H2>Log Out</H2>
			</div>

			<p class="flex-none my-2">
				I didn't say it was gonna be easy, Neo. I just said it would be the truth.
			</p>

			<div class="flex-none mt-auto">
				<ButtonRaw styling="bg-red-600 hover:bg-red-700" buttonName="Log out" />
			</div>
		</section>
	</form>
</div>
