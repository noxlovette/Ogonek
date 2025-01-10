<script lang="ts">
	import {
		Book,
		Bookmark,
		BookOpenText,
		House,
		ListTodo,
		MessageSquare,
		Settings,
		Video,
		WholeWord
	} from 'lucide-svelte';
	import { user } from '$lib/stores';
	import { page } from '$app/stores';
</script>

<footer class="md:hidden w-5/6 fixed bottom-2 text-brick-100 bg-brick-900 mt-auto rounded-full">
	<ul class="flex flex-row justify-between px-6 items-center">
		<a
			href="/u/tasks"
			class={$page.url.pathname === '/u/tasks' ? 'p-2 bg-brick-100/20 rounded-full' : 'px-2 py-4'}
		>
			<ListTodo class="size-6 " />
		</a>

		<a
			href="/u/lessons/bookmarked"
			class={$page.url.pathname === '/u/lessons/bookmarked'
				? 'p-2 bg-brick-100/20 rounded-full'
				: 'px-2 py-4'}
		>
			<Bookmark class="size-6 " />
		</a>

		<a
			href="/u/lessons"
			class=" {$page.url.pathname === '/u/lessons'
				? 'p-2 bg-brick-100/20 rounded-full'
				: 'px-2 py-4'}"
		>
			<BookOpenText class="size-6 " />
		</a>
		<a
			href="/u/dashboard"
			class=" {$page.url.pathname === '/u/dashboard'
				? 'p-2 bg-brick-100/20 rounded-full'
				: 'px-2 py-4'}"
		>
			<House class="size-6 rounded-full  " />
		</a>

		<a
			href="https://dictionary.cambridge.org/"
			target="_blank"
			rel="noopener noreferrer"
			class="px-2 py-4
                    "
		>
			<Book class="size-6 " />
		</a>
		<a href={$user.quizlet_url} class="px-2 py-4" target="_blank" rel="noopener noreferrer">
			<WholeWord class="size-6 " />
		</a>
		<a
			href="https://us05web.zoom.us/j/3661071003?pwd=RTlrUkRPaHJaakljZXQxaGpOYmdIZz09"
			target="_blank"
			rel="noopener noreferrer"
			class="px-2 py-4"
		>
			<Video class="size-6 " />
		</a>
	</ul>
</footer>
