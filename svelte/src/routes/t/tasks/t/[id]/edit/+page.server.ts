import type { Actions } from "@sveltejs/kit";
import { fail, redirect } from '@sveltejs/kit';

export const actions = {
    update: async ({ request, fetch }) => {
        const formData = await request.formData();
        const markdown = formData.get('markdown');
        const title = formData.get('title');
        const id = formData.get('id');
        const assignee = formData.get('assignee');

        let body = {
            id,
            title,
            markdown,
            assignee
        };

        console.log('Updating task:', body);

        try {
            const response = await fetch(`/axum/task/t/${formData.get('id')}`, {
                method: 'PATCH',
                body: JSON.stringify(body)
            });

            if (!response.ok) {
                const errorData = await response.json(); // Parse error details
                console.error('Error updating task:', errorData);
                return {
                    success: false,
                    error: errorData
                };
            }

            return redirect(303, `/t/tasks/t/${id}`);
        } catch (error) {
            console.error('Fetch error:', error);
            return {
                success: false,
                error: 'An error occurred while updating the task'
            };
        }
    },
    delete: async ({ request, fetch }) => {
        const formData = await request.formData();
        const id = formData.get('id');
        console.log('Deleting task:', id);

        const response = await fetch(`/axum/task/l/${id}`, {
            method: 'DELETE'
        });

        if (!response.ok) {
            const errorData = await response.json(); // Parse error details
            console.error('Error deleting task:', errorData);
            return {
                success: false,
                error: errorData
            };
        }

        redirect(303, '/t/tasks');

    }
} satisfies Actions;
