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

<div class="md:flex select-none flex-col hidden md:text-lg lg:text-xl xl:text-2xl font-serif w-1/5 px-4 py-2">
	<ul class="flex flex-col space-y-2 lg:space-y-3 xl:space-y-4">
		<a href="/s/dashboard" class="lg:px-2 lg:py-4 p-1 py-2 inline-flex">
			<House class="xl:size-8 lg:size-7 size-6 mr-2 " />
			Dashboard
		</a>

		<a href="/s/tasks" class="lg:px-2 lg:py-4 px-1 py-2 inline-flex">
			<ListTodo class="xl:size-8 lg:size-7 size-6 mr-2" />
			Tasks
		</a>

		<a href="/s/lessons/bookmarked" class="lg:px-2 lg:py-4 px-1 py-2 inline-flex">
			<Bookmark class="xl:size-8 lg:size-7 size-6 mr-2" />
			Bookmarks
		</a>

		<a
			href="https://us05web.zoom.us/j/3661071003?pwd=RTlrUkRPaHJaakljZXQxaGpOYmdIZz09"
			class="lg:px-2 lg:py-4 px-1 py-2 inline-flex"
			target="_blank"
			rel="noopener noreferrer"
		>
			<Video class="xl:size-8 lg:size-7 size-6 mr-2" />
			Zoom
		</a>
		<a
			href={$user.quizlet_url}
			class="lg:px-2 lg:py-4 px-1 py-2 inline-flex"
			target="_blank"
			rel="noopener noreferrer"
		>
			<WholeWord class="xl:size-8 lg:size-7 size-6 mr-2" />

			Quizlet
		</a>

		<a
			href="/s/settings"
			class="lg:px-2 lg:py-4 px-1 py-2 inline-flex
					"
		>
			<Settings class="xl:size-8 lg:size-7 size-6 mr-2" />

			Settings
		</a>
	</ul>
</div>

<style>
	a:hover {
		@apply bg-forest-700 transition-colors duration-300 rounded-lg text-forest-100;
	}
</style>
