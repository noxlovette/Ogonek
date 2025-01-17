import type { Actions } from "@sveltejs/kit";
import { fail, redirect, json } from '@sveltejs/kit';
import { randomUUID } from 'crypto';
import { writeFile } from 'fs/promises';
import path from 'path';
import { Readable } from 'stream';


export const actions = {
    update: async ({ request, fetch }) => {
        const formData = await request.formData();
        const markdown = formData.get('markdown');
        const title = formData.get('title');
        const id = formData.get('id');
        const assignee = formData.get('assignee');
        const dueDate = formData.get('dueDate') as string;
        const completed = formData.has('completed');

        // Handle the date conversion properly
        const dueDateWithTime = dueDate && dueDate !== ''
            ? new Date(`${dueDate}T23:59:59`).toISOString() // Need to make it ISO string for API
            : null;


        console.log('dueDateWithTime', dueDateWithTime);
        let body = {
            id,
            title,
            markdown,
            assignee,
            "dueDate": dueDateWithTime,
            completed,
        };

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

        redirect(303, `/t/tasks/t/${id}`);
    },
    delete: async ({ request, fetch }) => {
        const formData = await request.formData();
        const id = formData.get('id');
        const response = await fetch(`/axum/task/t/${id}`, {
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

    },
} satisfies Actions;
