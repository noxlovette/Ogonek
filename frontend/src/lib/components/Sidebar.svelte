<script lang="ts">
	import {user} from '$lib/stores';
	import { Hand, House, ListTodo, MessageSquare, Settings, Video, WholeWord } from 'lucide-svelte';
</script>

<div>
	<div
		class="flex flex-col w-64 h-screen text-sand-100 text-2xl font-serif  justify-between"
	>
		<div class="flex p-4">
			<h1 class=" text-4xl p-2">Firelight</h1>
		</div>

		<ul class="flex flex-col p-4 justify-between h-1/2">
			<a href="/u/dashboard" class="px-2 py-4 inline-flex">
				<House class="size-8 mr-2 " />
				Dashboard</a>

			<a href="/u/tasks" class="px-2 py-4 inline-flex">
				<ListTodo class="size-8 mr-2" />
				Tasks</a>

			<a href="/u/grok" class="px-2 py-4 inline-flex">
				<MessageSquare class="size-8 mr-2" />
				Grok</a>

			<a
				href="https://us05web.zoom.us/j/3661071003?pwd=RTlrUkRPaHJaakljZXQxaGpOYmdIZz09"
				class="px-2 py-4 inline-flex"
			>
			<Video class="size-8 mr-2" />
				Zoom
			</a>
			<a href="{$user.quizlet_url}" class="px-2 py-4 inline-flex">
				<WholeWord class="size-8 mr-2" />
				
				Quizlet </a>

			<a
				href="/u/settings"
				class="px-2 py-4 inline-flex
					">
					<Settings class="size-8 mr-2" />
					
					Settings</a
			>
		</ul>
		<div class="p-4">
			
			<h2 class="p-2 inline-flex">
				{#if user}
				<Hand class="size-8 mr-2" />
					{$user.username}
				{:else}
					Not logged in
				{/if}
			</h2>
		</div>
	</div>
</div>

<style>
	a:hover {
		@apply bg-sand-900/20 transition-colors duration-300 rounded-lg text-sand-100;
	}
</style>
