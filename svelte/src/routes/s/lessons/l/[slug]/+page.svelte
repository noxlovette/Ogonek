<script lang="ts">
	import { run } from 'svelte/legacy';

	import type { PageServerData } from './$types';
	import { marked } from 'marked';
	import { BookmarkMinus, BookmarkPlus } from 'lucide-svelte';
	import { enhance } from '$app/forms';
	import { notification } from '$lib/stores';

	interface Props {
		data: PageServerData;
		html: string;
	}

	let { data, html = $bindable() }: Props = $props();

	let bookmarked: boolean = $state(data.lesson.bookmarked);

	run(() => {
		bookmarked = data.lesson.bookmarked;
	});
	run(() => {
		html = marked.parse(data.lesson.content);
	});

	const handleToggle = async ({ result, update }) => {
		if (result.data) {
			if (bookmarked) {
				notification.set({ message: 'Bookmark Added', type: 'success' });
			} else {
				notification.set({ message: 'Bookmark Removed', type: 'success' });
			}
		} else {
			notification.set({
				message: result.data.error || 'Failed to Add Bookmark',
				type: 'error'
			});
		}
		update();
	};

	let date = $derived(new Date(data.lesson.manual_date || data.lesson.created_at));
	

	let formattedDate = $derived(date.toLocaleDateString('en-GB', {
		month: 'short',
		day: 'numeric',
		year: 'numeric'
	}));
	
</script>

<svelte:head>
	<title>Lesson from {formattedDate}</title>
</svelte:head>

<article class="flex flex-col text-lg size-full px-8 py-4">
	<div id="header" class="flex flex-col md:flex-row items-start justify-between">
		<h1 class="text-4xl font-bold">{formattedDate}</h1>
		<div class="flex flex-col md:border-2 md:px-2 md:py-1 py-4 rounded-lg border-sand-900/60">
			<h2><span class="text-sm opacity-60 font-bold mr-1">Topic:</span> {data.lesson.topic}</h2>
			<h3>
				<span class="text-sm opacity-60 font-bold mr-1">Category:</span>{data.lesson.category}
			</h3>
		</div>
	</div>
	<div class="markdown">{@html html}</div>

	<form action="?/bookmark" use:enhance={() => handleToggle} method="post">
		<input type="hidden" name="id" value={data.lesson.id} />
		<input type="hidden" name="bookmarked" value={bookmarked} />
		<button
			class="hover:bg-sand-900/60 hover:text-sand-100 transition-colors duration-300 rounded-lg my-8 p-2 inline-flex"
			onclick={() => (bookmarked = !bookmarked)}
		>
			{#if bookmarked}
				<BookmarkMinus class="size-8 mr-2" />
				Remove from bookmarks
			{:else}
				<BookmarkPlus class="size-8 mr-2" />
				Bookmark
			{/if}
		</button>
	</form>
</article>
