<script lang="ts">
    import { goto } from '$app/navigation';
    
    let username = '';
    let password = '';
    let isLoading = false;
    let error: string | null = null;
  
    async function handleLogin() {
      isLoading = true;
      error = null;
  
      try {
        const res = await fetch('/axum/auth/signin', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({
            username,
            pass: password // Matching your API's expected field name
          })
        });
  
        if (!res.ok) throw new Error('Invalid credentials');
  
        const { accessToken } = await res.json();
        
        // Store token (you might want to use a store instead)
        localStorage.setItem('token', accessToken);
        
        // Optional: Validate token immediately
        // const user = await validateAccess(accessToken);
        // if (!user) throw new Error('Token validation failed');
  
        // You might want to set up a user store here
        goto('/s/dashboard'); // or wherever you want to redirect after login
        
      } catch (err) {
        error = err instanceof Error ? err.message : 'Login failed';
      } finally {
        isLoading = false;
      }
    }
  </script>
  
  <form 
    on:submit|preventDefault={handleLogin}
    class="space-y-4 max-w-md mx-auto p-6"
  >
    <div class="space-y-2">
      <label for="username" class="block text-sm font-medium text-gray-700">
        Username
      </label>
      <input
        id="username"
        type="text"
        bind:value={username}
        class="w-full px-3 py-2 border rounded-md focus:ring-2 focus:ring-blue-500"
        required
      />
    </div>
  
    <div class="space-y-2">
      <label for="password" class="block text-sm font-medium text-gray-700">
        Password
      </label>
      <input
        id="password"
        type="password"
        bind:value={password}
        class="w-full px-3 py-2 border rounded-md focus:ring-2 focus:ring-blue-500"
        required
      />
    </div>
  
    {#if error}
      <div class="text-red-500 text-sm">{error}</div>
    {/if}
  
    <button
      type="submit"
      disabled={isLoading}
      class="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 
             disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
    >
      {#if isLoading}
        <span class="inline-block animate-spin mr-2">↻</span>
      {/if}
      {isLoading ? 'Signing in...' : 'Sign in'}
    </button>
  </form>