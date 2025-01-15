<script lang="ts">
    import { goto } from '$app/navigation';
    import { ValidateAccess } from '$lib/utils';

    let username = '';
    let password = '';
    let error = '';
    let loading = false;

    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();
        loading = true;
        error = '';

        try {
            const response = await fetch('/api/auth/signin', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
					'X-API-KEY': "oCvJe2zibUf6RC/l68hLslG3JBaRvGtCCoBfFSre+wY"
                },
                body: JSON.stringify({
                    username,
                    pass: password
                }),
                // Important for cookies!
                credentials: 'include'
            });

            if (!response.ok) {
                throw new Error('Login failed');
            }

            const { accessToken } = await response.json();
            const user = await ValidateAccess(accessToken);
            
            if (!user) {
                throw new Error('Invalid access token');
            }

            // Optional: Store user data in a store
            // userStore.set(user);

            // Redirect after successful login
            goto('/s/dashboard');
        } catch (err) {
            error = err instanceof Error ? err.message : 'Login failed';
            console.error('Signin error:', err);
        } finally {
            loading = false;
        }
    }
</script>

<form on:submit={handleSubmit} class="space-y-4">
    {#if error}
        <div class="text-red-500">{error}</div>
    {/if}
    
    <div>
        <input
            type="text"
            bind:value={username}
            placeholder="Username"
            class="w-full p-2 border rounded"
            required
        />
    </div>
    
    <div>
        <input
            type="password"
            bind:value={password}
            placeholder="Password"
            class="w-full p-2 border rounded"
            required
        />
    </div>
    
    <button
        type="submit"
        class="w-full p-2 bg-blue-500 text-white rounded"
        disabled={loading}
    >
        {loading ? 'Logging in...' : 'Login'}
    </button>
</form>