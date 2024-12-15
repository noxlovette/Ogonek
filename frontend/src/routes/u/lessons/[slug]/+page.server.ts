import type { Actions } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";


const VITE_API_URL = "http://backend:8000";

export const load: PageServerLoad = async ({params}) => {
  const { slug } = params;
  const lesson:App.Lesson = await fetch(`${VITE_API_URL}/api/lessons/${slug}/`).then((res) => res.json());

  return {
    lesson,
  };
};

export const actions = {
  bookmark: async ({ request }) => {
    const formData = await request.formData();

    let body = {
      id: formData.get("id"),
      bookmarked: formData.get("bookmarked"),
    };

    try {
      const response = await fetch(
        `${VITE_API_URL}/api/lessons/${formData.get("id")}/`,
        {
          method: "PATCH",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(body),
        }
      );

      if (!response.ok) {
        const errorData = await response.json(); // Parse error details
        console.error("Error updating lesson:", errorData);
        return {
          success: false,
          error: errorData,
        };
      }

      return {
        success: true,
        message: "Lesson updated successfully",
      };
    } catch (error) {
      console.error("Fetch error:", error);
      return {
        success: false,
        error: "An error occurred while updating the lesson",
      };
    }
  },
} satisfies Actions;