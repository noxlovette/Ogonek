<script lang="ts">
    import { enhance } from '$app/forms';
  
    let username = '';
    let password = '';
    let errorMessage = '';
    let successMessage = '';
  
    // We'll use Svelte's form enhancement for better user experience
  </script>
  
  <form method="POST" use:enhance action="?/login">
    <div>
      <label for="username">Username:</label>
      <input type="text" id="username" name="username" bind:value={username} required>
    </div>
    <div>
      <label for="password">Password:</label>
      <input type="password" id="password" name="password" bind:value={password} required>
    </div>
    {#if errorMessage}
      <p style="color: red;">{errorMessage}</p>
    {/if}
    {#if successMessage}
      <p style="color: green;">{successMessage}</p>
    {/if}
    <button type="submit">Login</button>
  </form>