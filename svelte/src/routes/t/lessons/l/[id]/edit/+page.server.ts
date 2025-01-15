import type { Actions } from "@sveltejs/kit";
import { fail, redirect } from '@sveltejs/kit';

export const actions = {
    update: async ({ request, fetch }) => {
        const formData = await request.formData();
        const markdown = formData.get('markdown');
        const title = formData.get('title');
        const id = formData.get('id');
        const topic = formData.get('topic');
        const assignee = formData.get('assignee');

        let body = {
            id,
            title,
            markdown,
            topic,
            assignee
        };

        console.log('Updating lesson:', body);

        
            const response = await fetch(`/axum/lesson/l/${formData.get('id')}`, {
                method: 'PATCH',
                body: JSON.stringify(body)
            });

            if (!response.ok) {
                const errorData = await response.json(); // Parse error details
                console.error('Error updating lesson:', errorData);
                return {
                    success: false,
                    error: errorData
                };
            }

            redirect(303, `/t/lessons/l/${id}`);
 
        
    },
    delete: async ({ request, fetch }) => {
        const formData = await request.formData();
        const id = formData.get('id');
        console.log('Deleting lesson:', id);

        const response = await fetch(`/axum/lesson/l/${id}`, {
            method: 'DELETE'
        });

        if (!response.ok) {
            const errorData = await response.json(); // Parse error details
            console.error('Error deleting lesson:', errorData);
            return {
                success: false,
                error: errorData
            };
        }

        redirect(303, '/t/lessons');

    }
} satisfies Actions;
