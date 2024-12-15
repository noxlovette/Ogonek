import type { LayoutServerLoad } from "../$types";

const VITE_API_URL = "http://backend:8000";

export const load: LayoutServerLoad = async () => {
    const response = await fetch(`${VITE_API_URL}/api/tasks/`);
    const lessonsResponse = await fetch

    const [tasks, lessons] = await Promise.all([
        fetch(`${VITE_API_URL}/api/tasks/`).then((res) => res.json()),
        fetch(`${VITE_API_URL}/api/lessons/`).then((res) => res.json()),
      ]);
    
    return {tasks, lessons};
    }