import { error, redirect } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "../$types";

const VITE_API_URL = "http://backend:8000";

export const actions = {
    create: async ({ cookies, request  }) => {
      const csrfToken = cookies.get("csrftoken");
      const formData = await request.formData();

      let body = {
        title: "Neuer Scheiß",
        processed_html: "",
        content: "",
        tags: [],
      }
      
        const response = await fetch(
          `${VITE_API_URL}/api/articles/`,
          {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
              Cookie: `csrftoken=${csrfToken}`,
            },
            credentials: "include",
            body: JSON.stringify(body),
          },
        );
  
        const data = await response.json();

        console.log(data);
        redirect(301, `/editor/${data.id}`);
        
    },
    
  } satisfies Actions;
  