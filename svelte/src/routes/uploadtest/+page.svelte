<script>
	let file;

	async function uploadFile() {
		if (!file) {
			alert('Please select a file first.');
			return;
		}

		const formData = new FormData();
		formData.append('file', file);

		const response = await fetch('/upload', {
			method: 'POST',
			body: formData
		});

		const result = await response.json();
		console.log(result);
		alert(result.message);
	}
</script>

<form on:submit|preventDefault={uploadFile}>
	<input type="file" on:change={(e) => (file = e.target.files[0])} />
	<button type="submit">Upload</button>
</form>
