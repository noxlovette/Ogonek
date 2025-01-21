<script lang="ts">
	import { onMount } from 'svelte';
	import { Download, ArrowLeft, BookOpen, Loader2 } from 'lucide-svelte';
	import { stripUUID } from '$lib/utils';

	let { data } = $props();
	const { body, headers, filename } = data;

	let isDownloading = $state(true);
	let downloadStarted = $state(false);

	onMount(() => {
		try {
			const blob = new Blob([body], { type: headers['Content-Type'] });
			const url = URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = stripUUID(filename);
			a.click();
			URL.revokeObjectURL(url);
			downloadStarted = true;
		} catch (e) {
			console.error(e);
			isDownloading = false;
		}
	});

	function downloadAgain() {
		isDownloading = true;
		const blob = new Blob([body], {
			type: headers['Content-Type'] || 'application/octet-stream'
		});
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = stripUUID(filename);
		a.click();
		URL.revokeObjectURL(url);
	}

	function goBack() {
		window.history.back();
	}
</script>

<div class=" flex items-center justify-center size-full">
	<div class="max-w-md w-full bg-white rounded-xl shadow-md p-8 space-y-6">
		<div class="flex items-center justify-center space-x-2">
			<BookOpen class="w-8 h-8 text-brick-500" />
			<h1 class="text-2xl font-bold text-milk-800">Homework Time! 📚</h1>
		</div>

		<div class="space-y-4">
			{#if isDownloading}
				<div class="flex flex-col items-center space-y-3">
					<Loader2 class="w-8 h-8 text-brick-500 animate-spin" />
					<p class="text-milk-600">Getting your homework ready...</p>
				</div>
			{:else}
				<div class="flex flex-col items-center space-y-3">
					<Download class="w-8 h-8 text-brick-500" />
					<p class="text-milk-600">
						{downloadStarted ? 'Almost there! 🚀' : 'Ready to download! 🎉'}
					</p>
				</div>
			{/if}

			<button
				onclick={downloadAgain}
				class="w-full bg-brick-500 hover:bg-brick-600 text-white font-medium py-2 px-4 rounded-lg transition-colors duration-200 flex items-center justify-center space-x-2"
			>
				<Download class="w-4 h-4" />
				<span>Download Again</span>
			</button>

			<button
				onclick={goBack}
				class="w-full mt-2 bg-milk-100 hover:bg-milk-200 text-milk-700 font-medium py-2 px-4 rounded-lg transition-colors duration-200 flex items-center justify-center space-x-2"
			>
				<ArrowLeft class="w-4 h-4" />
				<span>Go Back</span>
			</button>
		</div>

		<p class="text-sm text-milk-500 text-center">
			Pro tip: Your teacher won't believe how fast you did this! 😉
		</p>
	</div>
</div>
