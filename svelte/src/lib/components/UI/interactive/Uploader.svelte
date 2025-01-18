<script lang="ts">
	import { enhance } from '$app/forms';
	import { notification } from '$lib/stores';
	import { Upload, Loader2 } from 'lucide-svelte';
	import { fade, scale } from 'svelte/transition';

	let { id } = $props();

	let isDragging = $state(false);
	let isUploading = $state(false);
	let fileName = $state('');
	let fileInput: HTMLInputElement;

	function handleDragOver(e: DragEvent) {
		e.preventDefault();
		isDragging = true;
	}

	function handleDragLeave() {
		isDragging = false;
	}

	function handleDrop(e: DragEvent) {
		e.preventDefault();
		isDragging = false;
		const file = e.dataTransfer?.files[0];
		if (file) handleFileSelect(file);
	}

	function handleFileSelect(file: File) {
		fileName = file.name;
	}

	function handleChange(e: Event) {
		const file = (e.target as HTMLInputElement).files?.[0];
		if (file) handleFileSelect(file);
	}
</script>

<form
	method="post"
	enctype="multipart/form-data"
	action="?/upload"
	class=""
	use:enhance={({ formData, cancel }) => {
		if (!formData.has('file')) {
			cancel();
			notification.set({
				message: 'Please select a file first',
				type: 'error'
			});
			return;
		}

		isUploading = true;

		return async ({ result }) => {
			isUploading = false;

			if (result.type === 'success') {
				fileName = '';
				notification.set({
					message: 'File uploaded successfully!',
					type: 'success'
				});
			} else if (result.type === 'failure') {
				notification.set({
					message: String(result.data?.message ?? 'An error occurred'),
					type: 'error'
				});
			} else {
			notification.set({
					message: 'An unknown error occurred',
					type: 'error'
				});
			}
		};
	}}
>
	<input type="hidden" name="id" value={id} />
	<!-- Upload Area -->
	<div
		onclick={() => fileInput.click()}
		onkeydown={(e) => (e.key === 'Enter' || e.key === ' ' ? fileInput.click() : null)}
		ondragover={handleDragOver}
		ondragleave={handleDragLeave}
		ondrop={handleDrop}
		role="button"
		tabindex="0"
		aria-label="File upload area"
		class="relative rounded-lg border-2 border-dashed p-12
			 flex flex-col items-center justify-center
			 cursor-pointer transition-colors duration-200
			 {isDragging
			? 'border-brick-700 bg-brick-100'
			: 'border-milk-300 hover:border-milk-400 bg-milk-50'}"
	>
		<input bind:this={fileInput} type="file" name="file" onchange={handleChange} class="hidden" />

		{#if isUploading}
			<div class="flex flex-col items-center gap-3" in:scale={{ duration: 200 }}>
				<Loader2 class="w-10 h-10 text-brick-500 animate-spin" />
				<p class="text-milk-600">Uploading {fileName}...</p>
			</div>
		{:else}
			<div class="flex flex-col items-center gap-3" in:fade>
				<Upload class="w-10 h-10 {fileName ? 'text-brick-500' : 'text-milk-400'}" />
				<div class="text-center">
					<p class="text-milk-600">
						{fileName || 'Drag and drop your file here, or click to select'}
					</p>
				</div>
			</div>
		{/if}
	</div>

	<!-- Submit Button -->
	<button
		type="submit"
		disabled={isUploading || !fileName}
		class="mt-4 w-full py-2 px-4 rounded-md
			 flex items-center justify-center gap-2
			 transition-colors duration-200
			 {isUploading || !fileName
			? 'bg-milk-200 text-milk-500 cursor-not-allowed'
			: 'bg-brick-500 text-white hover:bg-brick-600'}"
	>
		{#if isUploading}
			<Loader2 class="w-4 h-4 animate-spin" />
			Uploading...
		{:else}
			Upload File
		{/if}
	</button>
</form>
