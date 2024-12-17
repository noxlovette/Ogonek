<script>
    import { onMount, onDestroy } from 'svelte';
    
    let messages = [];
    let eventSource;
  
    onMount(() => {
      eventSource = new EventSource('/api/claude-stream');
  
      eventSource.onmessage = (event) => {
        const data = JSON.parse(event.data);
        if (data.text) {
          messages = [...messages, data.text];
        }
      };
  
      eventSource.onerror = (error) => {
        console.error('EventSource failed:', error);
        // Optionally, try to reconnect after some delay
        setTimeout(() => {
          eventSource.close();
          eventSource = new EventSource('/api/claude-stream');
        }, 3000); // Reconnect after 3 seconds
      };
  
      eventSource.onopen = () => {
        console.log('EventSource connection opened');
      };
    });
  
    onDestroy(() => {
      if (eventSource) {
        eventSource.close();
      }
    });
  </script>
  
  <div>
    {#each messages as message}
      <p>{message}</p>
    {/each}
  </div>