import type { Actions } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";


const VITE_API_URL = "http://backend:8000";

export const load: PageServerLoad = async ({params}) => {
  const { slug } = params;
  const article = await fetch(`${VITE_API_URL}/api/articles/${slug}/`).then((res) => res.json());

  return {
    article,
  };
};

export const actions = {
  update: async ({ request }) => {
    const formData = await request.formData();

    let body = {
      id: formData.get("id"),
      title: formData.get("title"),
      content: formData.get("content"),
      processed_html: formData.get("processed_html"),
      category: formData.get("category"),
      tags: formData.getAll("tags"),
      difficulty: formData.get("difficulty"),
    };

    try {
      const response = await fetch(
        `${VITE_API_URL}/api/articles/${formData.get("id")}/`,
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
        console.error("Error updating article:", errorData);
        return {
          success: false,
          error: errorData,
        };
      }

      return {
        success: true,
        message: "Article updated successfully",
      };
    } catch (error) {
      console.error("Fetch error:", error);
      return {
        success: false,
        error: "An error occurred while updating the article",
      };
    }
  },
} satisfies Actions;
