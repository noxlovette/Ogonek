import type { Actions } from './$types';
import { redirect, fail } from '@sveltejs/kit';

export const actions: Actions = {
    default: async ({ fetch }) => {
        const response = await fetch("/axum/auth/invite", { method: "POST" });
        
        if (!response.ok) {
            return fail(400, { error: 'Failed to generate invite link' });
        }

        const data = await response.json();
        console.log('Invite link:', data);
        
        // If your Axum endpoint returns something like:
        // Ok(Json("https://your-app.com/signup?invite=abc123..."))
        // Then 'data' would be that URL string

        return { 
            success: true,
            data 
        };
    }
};