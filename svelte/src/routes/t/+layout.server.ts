import type { LayoutServerLoad } from "./$types";
import type { Lesson, Task, Student } from '$lib/types'
export const load: LayoutServerLoad = async ({ fetch }) => {

    const [students, lessons, tasks] = await Promise.all([
        fetch("/axum/student").then((res) => res.json() as Promise<Student[]>),
        fetch("/axum/lesson").then((res) => res.json() as Promise<Lesson[]>),
        fetch("/axum/task").then((res) => res.json() as Promise<Task[]>),
    ]);

    return {
        students,
        lessons,
        tasks
    };
}