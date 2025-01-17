<script lang="ts">
	import { enhance } from '$app/forms';
</script>

<div class="items-start flex flex-row space-x-8">
	<!-- User Settings -->
	<div class="bg-milk-50 rounded-lg">
		<h2 class="text-2xl font-semibold text-brick-800 mb-4">User Settings</h2>
		<form method="POST" class="space-y-4" use:enhance action="?/updateUser">
			<div class="space-y-2">
				<label for="email" class="block text-sm font-medium text-brick-700"> Email </label>
				<input
					type="email"
					id="email"
					name="email"
					class="w-full px-3 py-2 border border-milk-200 rounded-md focus:outline-none focus:ring-2 focus:ring-brick-500"
				/>
			</div>

			<div class="space-y-2">
				<label for="username" class="block text-sm font-medium text-brick-700"> Username </label>
				<input
					type="text"
					id="username"
					name="username"
					class="w-full px-3 py-2 border border-milk-200 rounded-md focus:outline-none focus:ring-2 focus:ring-brick-500"
				/>
			</div>

			<div class="space-y-2">
				<label for="name" class="block text-sm font-medium text-brick-700"> Name </label>
				<input
					type="text"
					id="name"
					name="name"
					class="w-full px-3 py-2 border border-milk-200 rounded-md focus:outline-none focus:ring-2 focus:ring-brick-500"
				/>
			</div>

			<button
				type="submit"
				class="w-full bg-brick-600 text-white py-2 px-4 rounded-md hover:bg-brick-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-brick-500"
			>
				Update User
			</button>
		</form>
	</div>

	<!-- Profile Settings -->
	<div class="bg-milk-50 rounded-lg">
		<h2 class="text-2xl font-semibold text-brick-800 mb-4">Profile Settings</h2>
		<form class="space-y-4" use:enhance method="POST" action="?/updateProfile">
			<div class="space-y-2">
				<label for="quizlet" class="block text-sm font-medium text-brick-700"> Quizlet URL </label>
				<input
					type="url"
					id="quizlet"
					name="quizlet"
					class="w-full px-3 py-2 border border-milk-200 rounded-md focus:outline-none focus:ring-2 focus:ring-brick-500"
				/>
			</div>

			<div class="space-y-2">
				<label for="zoom" class="block text-sm font-medium text-brick-700"> Zoom URL </label>
				<input
					type="url"
					id="zoom"
					name="zoom"
					class="w-full px-3 py-2 border border-milk-200 rounded-md focus:outline-none focus:ring-2 focus:ring-brick-500"
				/>
			</div>

			<button
				type="submit"
				class="w-full bg-brick-600 text-white py-2 px-4 rounded-md hover:bg-brick-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-brick-500"
			>
				Update Profile
			</button>
		</form>
	</div>
</div>
