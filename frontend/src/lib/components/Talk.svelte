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
        if (messages[messages.length - 1]?.role === 'assistant') {
          messages = messages.map((msg, index) => 
            index === messages.length - 1 ? { ...msg, text: msg.text + data.text } : msg
          );
        } else {
          messages = [...messages, { role: 'assistant', text: data.text }];
        }
      }
    };

    eventSource.onerror = (error) => {
      console.error('EventSource failed:', error);
      console.log('Connection closed, attempting to reconnect...');
        console.error('EventSource failed:', error);
        console.log('EventSource readyState:', eventSource.readyState);
        console.log('EventSource lastEventId:', eventSource.lastEventId);
        // Log the error event object to see more details if available:
        console.log('Error event:', error);
        
      if (eventSource.readyState === EventSource.CLOSED) {
        console.log('Connection closed, attempting to reconnect...');
        console.error('EventSource failed:', error);
        console.log('EventSource readyState:', eventSource.readyState);
        console.log('EventSource lastEventId:', eventSource.lastEventId);
        // Log the error event object to see more details if available:
        console.log('Error event:', error);
        // Reconnect strategy: 
        // 1. Close current connection if it's not already closed
        // 2. Wait a bit before creating a new one to avoid immediate loop
        setTimeout(() => {
          eventSource.close();
       //   eventSource = new EventSource('/api/claude-talk');
        }, 1000); // 1 second delay
      }
    };

    eventSource.onopen = () => {
      console.log('EventSource connection opened');
    };
  });

  const sendMessage = async () => {
    if (newMessage.trim() === '') return;

    messages = [...messages, { role: 'user', text: newMessage }];
    newMessage = '';

    await fetch('/api/claude-talk', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ message: messages[messages.length - 1].text }),
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
  <input bind:value={newMessage} on:keydown={(e) => e.key === 'Enter' && sendMessage()} placeholder="Type your message...">
  <button on:click={sendMessage}>Send</button>
</div>