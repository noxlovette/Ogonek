import type { Actions } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { error } from "@sveltejs/kit";

const VITE_API_URL = "http://backend:8000";


export const actions = {
  completed: async ({ request, cookies }) => {
    const formData = await request.formData();
    const sessionid = cookies.get('sessionid');
    const csrfToken = cookies.get('csrftoken');

    let body = {
      id: formData.get("id"),
      completed: formData.get("completed"),
    };

    try {
      const headers = {
        'Cookie': `sessionid=${sessionid}; csrftoken=${csrfToken}`,
        'Content-Type': "application/json",
        'X-CSRFToken': csrfToken,
      };

      // Update the task first
      const updateResponse = await fetch(
        `${VITE_API_URL}/api/tasks/${formData.get("id")}/`,
        {
          method: "PATCH",
          headers: headers,
          body: JSON.stringify(body),
        }
      );

      if (!updateResponse.ok) {
        const errorData = await updateResponse.json(); // Parse error details
        console.error("Error updating task:", errorData);
        return {
          success: false,
          error: errorData,
        };
      }

      // Then fetch all tasks
      const [tasks] = await Promise.all([
        fetch(`${VITE_API_URL}/api/tasks/`, {
          headers: {
            'Cookie': `sessionid=${sessionid}; csrftoken=${csrfToken}`
          }
        }).then((res) => res.json())
      ]);

      return {
        success: true,
        message: "Task updated successfully",
        tasks: tasks, // Return the updated list of tasks
      };
    } catch (error) {
      console.error("Fetch error:", error);
      return {
        success: false,
        error: "An error occurred while updating or fetching tasks",
      };
    }
  },
  download: async ({ request, cookies }) => {
    const formData = await request.formData();
    const sessionid = cookies.get('sessionid');
    const csrfToken = cookies.get('csrftoken');
    const url = `${formData.get("file")}`;
    

    try {
      const response = await fetch(
        url,
        {
          method: "GET",
          headers: {
            "Content-Type": "application/json",
            Cookie: `sessionid=${sessionid}; csrftoken=${csrfToken}`,
            "X-CSRFToken": csrfToken,
          },
        }
      );

      if (!response.ok) {
        const errorData = await response.json(); // Parse error details
        console.error("Error downloading task:", errorData);
        return {
          success: false,
          error: errorData,
        };
      }

      const blob = await response.blob();
    return {
      file: {
        blob: blob,
        name: 'roses-center.svg' // or dynamically get from headers
      }
    };
    } catch (error) {
      console.error("Fetch error:", error);
      return {
        success: false,
        error: "An error occurred while downloading the task",
      };
    }
  }
} satisfies Actions;