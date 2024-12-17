<script>
    import { onMount, onDestroy } from 'svelte';

    let messages = [];
    let newMessage = '';
    let eventSource;

    onMount(() => {
        eventSource = new EventSource('/api/claude-talk');

        eventSource.onmessage = (event) => {
            const data = JSON.parse(event.data);
            if (data.text) {
                messages = [...messages, { role: 'assistant', text: data.text }];
            }
        };

        eventSource.onerror = (error) => {
            console.error('EventSource failed:', error);
        };
    });

    const sendMessage = async () => {
        if (newMessage.trim() === '') return;

        messages = [...messages, { role: 'user', text: newMessage }];
        newMessage = '';

        await fetch('/api/claude-talk', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ message: newMessage })
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