import { fail, redirect, type Actions } from '@sveltejs/kit';

export const actions: Actions = {
    new: async ({ fetch, request }) => {

        const body = {
            title: "New Task",
            markdown: "## Try adding some content here",
        }

        console.log("new triggered");

        const response = await fetch(`/axum/task`, {
            method: 'POST',
            body: JSON.stringify(body)
        });

        const { id } = await response.json();

        console.log(response);

        if (response.ok) {
            return redirect(301, `/t/tasks/t/${id}`);
        }

    }
};