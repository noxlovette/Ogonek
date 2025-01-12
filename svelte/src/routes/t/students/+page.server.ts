import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch }) => {

    let students = [];
    students = await fetch("/axum/student").then((res) => res.json())

    return {
        students
    }
};