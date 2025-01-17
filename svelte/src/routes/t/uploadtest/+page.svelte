<!-- components/FileUpload.svelte -->
<script lang="ts">
	let uploadedFile: string | null = null; // Store the filename after upload

	async function handleUpload(event: Event) {
		const form = event.target as HTMLFormElement;
		const formData = new FormData(form);

		try {
			const response = await fetch('/t/upload', {
				method: 'POST',
				body: formData
			});

			if (!response.ok) throw new Error('Upload failed');

			const result = await response.json();
			uploadedFile = result.filename; // Save the filename
			console.log('File uploaded:', uploadedFile);
		} catch (err) {
			console.error('Upload error:', err);
		}
	}
</script>

<form on:submit|preventDefault={handleUpload} class="flex flex-col gap-4 p-4 border rounded-lg">
	<input
		type="file"
		name="file"
		class="file:mr-4 file:py-2 file:px-4 file:rounded-lg file:border-0 file:bg-blue-500 file:text-white hover:file:bg-blue-600"
	/>
	<button type="submit" class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600">
		Upload
	</button>
</form>

<!-- Show download button after successful upload -->
{#if uploadedFile}
	<div class="mt-4">
		<p class="text-green-600 mb-2">File uploaded successfully!</p>
		<a
			href="/uploads/{uploadedFile}"
			download
			class="inline-block px-4 py-2 bg-green-500 text-white rounded-lg hover:bg-green-600"
		>
			Download File
		</a>
	</div>
{/if}
