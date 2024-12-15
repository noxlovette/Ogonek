<script lang="ts">
	import { getContext } from 'svelte';

	const user = getContext('user');
</script>

<div>
	<div
		class="flex flex-col w-64 h-screen bg-sand-900/70 text-sand-50 text-2xl font-serif shadow-lg border-r border-sand-900/10"
	>
		<div class="flex h-18 bg-sand-950/80 p-4">
			<h1 class=" text-3xl p-2">Firelight</h1>
		</div>

		<ul class="flex flex-col p-4 shadow-inner">
			<a href="/" class="p-2">Dashboard</a>

			<a href="/tasks" class="p-2">Tasks</a>

			<a href="/grok" class="p-2">Grok</a>

			<a
				href="https://us05web.zoom.us/j/3661071003?pwd=RTlrUkRPaHJaakljZXQxaGpOYmdIZz09"
				class="p-2"
			>
				Zoom
			</a>
			<a href="https://quizlet.com" class="p-2"> Quizlet </a>
		</ul>
		<div class="mt-auto p-4">
			<a
				href="/settings"
				class="p-2
					">Settings</a
			>
			<h2>
				{#if user}
					<p class="p-2">Logged in as {user.email}</p>
				{:else}
					<p class="p-2">Not logged in</p>
				{/if}
			</h2>
		</div>
	</div>
</div>

<style>
	a:hover {
		@apply bg-sand-900/70 transition-colors duration-300 rounded-lg text-sand-100;
	}
</style>
