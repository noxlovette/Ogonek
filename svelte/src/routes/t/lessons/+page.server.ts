import { fail, redirect, type Actions } from '@sveltejs/kit';

export const actions: Actions = {
    new: async ({ fetch, request }) => {

        const body = {
            title: "New Lesson",
            markdown: "## Try adding some content here",
            topic: "General",
        }


        const response = await fetch(`/axum/lesson`, {
            method: 'POST',
            body: JSON.stringify(body)
        });

        const { id } = await response.json();


        if (response.ok) {
            return redirect(301, `/t/lessons/l/${id}/edit`);
        }

    }
};