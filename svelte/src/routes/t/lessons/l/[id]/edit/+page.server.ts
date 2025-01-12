import type { Actions } from "@sveltejs/kit";

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

        try {
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
        } catch (error) {
            console.error('Fetch error:', error);
            return {
                success: false,
                error: 'An error occurred while updating the lesson'
            };
        }
    }
} satisfies Actions;
