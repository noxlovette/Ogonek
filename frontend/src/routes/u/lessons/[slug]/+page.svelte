<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageServerData } from '../$types';
	import { marked } from 'marked';
	import { Bookmark, BookmarkMinus, BookmarkPlus } from 'lucide-svelte';
	import { enhance } from '$app/forms';
	import { notification } from '$lib/stores';

	export let data: PageServerData;
	export let html: string;

	let lesson = data.lesson;
	let bookmarked: boolean = data.lesson.bookmarked;

	$: bookmarked = data.lesson.bookmarked;
	$: html = marked.parse(data.lesson.content);

	const handleToggle = async ({ result, update }) => {
		if (result.data.result.success) {
			notification.set({ message: 'Bookmark Added', type: 'success' });
		} else {
			notification.set({
				message: result.data.error || 'Failed to Add Bookmark',
				type: 'error'
			});
		}
		update();
	};

	let date;
	$:date = new Date(data.lesson.created_at);


	let formattedDate;
	$: formattedDate = date.toLocaleDateString('en-GB', {
		month: 'short',
		day: 'numeric',
		year: 'numeric'
	});
</script>

<article class="flex flex-col text-lg size-full">
	<div id="header" class="flex flex-row items-start justify-between">
		<h1 class="text-4xl font-bold">{formattedDate}</h1>
		<div class="flex flex-col border-2 px-2 py-1 rounded-lg border-sand-900/60">
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
				class="hover:bg-sand-900/60 hover:text-sand-100 transition-colors duration-300 rounded-lg my-8 p-2 mb-32 inline-flex" on:click={() => (bookmarked = !bookmarked)}
			>
      {#if bookmarked}
      <BookmarkMinus class="size-8 mr-2" />
      Remove from bookmarks
      {:else}
      <BookmarkPlus class="size-8 mr-2" />
					Save the lesson for later
				{/if}
			</button>
		</form>

</article>
