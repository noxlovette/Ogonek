<script>
    import Chat from '$lib/components/Chat.svelte';
import { onMount } from 'svelte';
    
    let message = '';
  
    onMount(async () => {
      const response = await fetch('/api/claude-http');
      if (response.ok) {
        const data = await response.json();
        message = data.message || 'No content';
        console.log('Received message:', message);
      } else {
        message = 'Error fetching data';
      }
    });
  </script>
  
  <div>


      <h1>HTTP</h1>
      <p>{message}</p>
    </div>

    <Chat />