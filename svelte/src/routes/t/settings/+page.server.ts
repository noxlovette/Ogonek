import { fail, redirect, type Actions } from "@sveltejs/kit";
import { ValidateAccess } from "$lib/utils";
import type { Profile } from "$lib/types";
export const actions = {
    update: async ({ request, fetch }) => {
        console.debug("updateProfile");
        const formData = await request.formData();
        let zoomUrl = undefined;
        let quizletUrl = undefined;
        if (formData.has("zoom")) {
            zoomUrl = formData.get('zoom');
        }
        if (formData.has('quizlet')) {
            quizletUrl = formData.get('quizlet');
        }

        if (!zoomUrl && !quizletUrl) {
            fail(400, { message: "No fields provided" });
        }

        const profileBody = {
            quizletUrl: formData.get('quizlet'),
            zoomUrl: formData.get('zoom'),
            bio: formData.get('bio'),
            avatar_url: formData.get('avatarUrl'),
        };

        const userBody = {
            email: formData.get('email'),
            username: formData.get('username'),
            name: formData.get('name'),
        };

        const responseProfile = await fetch("/axum/profile", {
            method: "PATCH",
            body: JSON.stringify(profileBody),
        })


        console.debug(profileBody);
        console.log(userBody);

        const _ = await fetch("/axum/user", {
            method: "PATCH",
            body: JSON.stringify(userBody),
        })


        const responseRefresh = await fetch("/auth/refresh");
        if (!responseRefresh.ok) {
            return fail(500, { message: "Something's wrong" })
        }

        const profile: Profile = await responseProfile.json();

        const { accessToken } = await responseRefresh.json();
        const user = await ValidateAccess(accessToken);

        if (!user) {
            return fail(500, { message: "Invalid Token" })
        }

        return {
            success: true,
            profile,
            user,
            message: "Profile updated successfully",
        };
    },
    logout: async (event) => {
        event.cookies.delete('accessToken', { path: "/" })
        event.cookies.delete('refreshToken', { path: "/" })
        throw redirect(301, "/")
    }
} satisfies Actions;