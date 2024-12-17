<script>
	import { user } from '$lib/stores';
	import { onMount, onDestroy } from 'svelte';

	let messages = [];
	let newMessage = '';
	let eventSource;

	let clientId = null; // Store client ID here

    onMount(() => {
        fetch('/api/claude-talk', { method: 'GET' })
            .then(response => {
                clientId = response.headers.get('X-Client-ID'); // Get client ID from response
                console.log('Client ID:', clientId);
                eventSource = new EventSource('/api/claude-talk');

                // ... rest of your onMount logic
            })
            .catch(error => console.error('Error fetching client ID:', error));
    });

    const sendMessage = async () => {
        if (newMessage.trim() === '') return;

        messages = [...messages, { role: 'user', text: newMessage }];
        newMessage = '';
        await fetch('/api/claude-talk', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'X-Client-ID': clientId
            },
            body: JSON.stringify({ message: messages[messages.length - 1].text })
        });
    };

    onDestroy(() => {
        if (eventSource) {
            eventSource.close();
        }
    });
</script>

<div>
	{#each messages as message}
		<p>{message.role}: {message.text}</p>
	{/each}
	<input
		bind:value={newMessage}
		on:keydown={(e) => e.key === 'Enter' && sendMessage()}
		placeholder="Type your message..."
	/>
	<button on:click={sendMessage}>Send</button>
</div>
