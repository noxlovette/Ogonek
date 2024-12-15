import type { LayoutServerLoad } from "./$types";

const VITE_API_URL = "http://backend:8000";

export const load: LayoutServerLoad = async () => {
    const response = await fetch(`${VITE_API_URL}/api/tasks/`);
    const tasks = await response.json();
    return {tasks};
    }