import type { LayoutServerLoad } from "./$types";
export const load: LayoutServerLoad = async ({ fetch }) => {

    const [students, lessons, tasks] = await Promise.all([
        fetch("/axum/student").then((res) => res.json()),
        fetch("/axum/lesson").then((res) => res.json()),
        fetch("/axum/task").then((res) => res.json()),
    ]);

    return {
        students,
        lessons,
        tasks
    };
}