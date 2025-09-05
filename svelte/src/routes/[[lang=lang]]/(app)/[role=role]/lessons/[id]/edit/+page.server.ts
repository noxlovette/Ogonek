import { z } from "$lib";
import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { unsplash } from "$lib/server";
import type { UpsertPhoto } from "$lib/types";
import type { Actions } from "@sveltejs/kit";
import { fail, redirect } from "@sveltejs/kit";
import type { Basic as Photo } from "unsplash-js/dist/methods/photos/types";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
  if (params.role !== "t") {
    throw redirect(301, "/unauthorised");
  }
};

export const actions = {
  update: async ({ request, fetch, params }) => {
    const { id } = params;

    if (!id) {
      return fail(500);
    }
    const formData = await request.formData();
    const data = Object.fromEntries(formData);
    const body = z.updateLessonBody.safeParse(data).data;

    console.log(body);
    const response = await fetch(routes.lessons.lesson(id), {
      method: "PATCH",
      body: JSON.stringify(body),
    });

    if (!response.ok) {
      const errorData = await response.json();
      logger.error({ errorData, id }, "Error updating lesson axum-side");
      return {
        success: false,
        error: errorData,
      };
    }

    throw redirect(303, `/t/lessons/${id}`);
  },
  delete: async ({ fetch, params }) => {
    const { id } = params;
    if (!id) {
      return fail(500);
    }
    const response = await fetch(routes.lessons.lesson(id), {
      method: "DELETE",
    });

    if (!response.ok) {
      const errorData = await response.json();
      logger.error("Error deleting lesson axum-side", errorData);
      return {
        success: false,
        error: errorData,
      };
    }

    return redirect(303, "/t/lessons");
  },
  unsplash: async ({ request }) => {
    const formData = request.formData();
    const q = (await formData).get("q") as string | null;

    console.log(q);
    let photos: Photo[] = [];
    if (q) {
      await unsplash.search
        .getPhotos({
          query: q,
        })
        .then((result) => {
          if (result.type === "success") {
            photos = result.response.results;
          } else {
            console.log(result);
          }
        });
    }

    console.log(photos);

    return { photos };
  },
  addPhoto: async ({ request, fetch, params }) => {
    const { id } = params;
    if (!id) {
      return fail(400);
    }
    const data = await request.formData();

    const photoId = data.get("photoId") as string;
    if (!photoId) {
      return fail(400);
    }
    const { response } = await unsplash.photos.get({ photoId });

    if (response) {
      const rawBody: UpsertPhoto = {
        unsplashId: response.id,
        altDescription: response.alt_description,
        urls: response.urls,
        user: {
          username: response.user.username,
          name: response.user.name,
        },
      };
      const body = z.upsertPhotoBody.parse(rawBody);
      const axumResponse = await fetch(routes.lessons.upsert_photo(id), {
        method: "POST",
        body: JSON.stringify(body),
      });

      if (!axumResponse.ok) {
        return fail(500, { photo: true });
      }

      /**
       * Track the download as per Unsplash's guidelines:
       * https://help.unsplash.com/en/articles/2511258-guideline-triggering-a-download
       **/
      unsplash.photos.trackDownload({
        downloadLocation: response.links.download_location,
      });
    }
  },
  removePhoto: async ({ fetch, params }) => {
    const { id } = params;
    if (!id) {
      return fail(500);
    }
    const response = await fetch(routes.lessons.upsert_photo(id), {
      method: "DELETE",
    });

    if (!response.ok) {
      return fail(500);
    }

    return { success: true };
  },
} satisfies Actions;
