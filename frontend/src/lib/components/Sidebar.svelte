<script lang="ts">
	import { user, translations, language } from '$lib/stores';
	import {
		Bookmark,
		Hand,
		House,
		ListTodo,
		MessageSquare,
		Settings,
		Video,
		WholeWord
	} from 'lucide-svelte';
</script>

<div
	class="md:flex select-none flex-col hidden md:text-lg lg:text-xl xl:text-2xl min-h-screen w-1/3 text-sand-100 font-serif"
>
	<div class="flex p-4">
		<h1 class="text-2xl lg:text-3xl xl:text-4xl p-2">Firelight</h1>
	</div>

	<ul class="flex flex-col p-4 space-y-2 lg:space-y-3 xl:space-y-4">
		<a href="/u/dashboard" class="lg:px-2 lg:py-4 px-1 py-2 inline-flex">
			<House class="xl:size-8 lg:size-7 size-6 mr-2 " />
			{$translations.dashboard[$language]}</a
		>

		<a href="/u/tasks" class="lg:px-2 lg:py-4 px-1 py-2 inline-flex">
			<ListTodo class="xl:size-8 lg:size-7 size-6 mr-2" />
			{$translations.tasks[$language]}</a
		>

		<a href="/u/lessons/bookmarked" class="lg:px-2 lg:py-4 px-1 py-2 inline-flex">
			<Bookmark class="xl:size-8 lg:size-7 size-6 mr-2" />
			{$translations.bookmarks[$language]}</a
		>

		<a
			href="https://us05web.zoom.us/j/3661071003?pwd=RTlrUkRPaHJaakljZXQxaGpOYmdIZz09"
			class="lg:px-2 lg:py-4 px-1 py-2 inline-flex"
		>
			<Video class="xl:size-8 lg:size-7 size-6 mr-2" />
			Zoom
		</a>
		<a href={$user.quizlet_url} class="lg:px-2 lg:py-4 px-1 py-2 inline-flex">
			<WholeWord class="xl:size-8 lg:size-7 size-6 mr-2" />

			Quizlet
		</a>

		<a
			href="/u/settings"
			class="lg:px-2 lg:py-4 px-1 py-2 inline-flex
					"
		>
			<Settings class="xl:size-8 lg:size-7 size-6 mr-2" />

			{$translations.settings[$language]}</a
		>
	</ul>
	<div class="p-4 mt-auto">
		<h2 class="p-2 inline-flex">
			{#if user}
				<Hand class="xl:size-8 lg:size-7 size-6 mr-2" />
				{$user.username}
			{:else}
				Not logged in
			{/if}
		</h2>
	</div>
</div>

<style>
	a:hover {
		@apply bg-sand-900/20 transition-colors duration-300 rounded-lg text-sand-100;
	}
</style>
