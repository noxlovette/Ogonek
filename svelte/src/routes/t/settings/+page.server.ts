import { fail, type Actions } from "@sveltejs/kit";

export const actions = {
    updateProfile: async ({ request, fetch }) => {
        console.debug("updateProfile");
        const formData = await request.formData();
        const zoomUrl = formData.get('zoom');
        const quizletUrl = formData.get('quizlet');

        if (!zoomUrl && !quizletUrl) {
            fail(400, { message: "No fields provided" });
        }

        console.log(formData);
        const body = {
            quizletUrl: formData.get('quizlet'),
            zoomUrl: formData.get('zoom'),
            bio: formData.get('bio'),
            avatar_url: formData.get('avatarUrl'),
        };

        const response = await fetch("/axum/profile", {
            method: "PATCH",
            body: JSON.stringify(body),
        })

        console.debug(response);

        if (!response.ok) {
            fail(500, { message: "Failed to update profile" });
        }

        return {
            success: true,
            message: "Profile updated successfully",
        };
    },
    updateUser: async ({ request, fetch }) => {
        const formData = await request.formData();

        const body = {
            email: formData.get('email'),
            username: formData.get('username'),
            password: formData.get('password'),
        };

        const response = await fetch("/axum/user", {
            method: "PATCH",
            body: JSON.stringify(body),
        })

        console.debug(response);

        if (!response.ok) {
            fail(500, { message: "Failed to update user" });
        }

        return {
            success: true,
            message: "User updated successfully",

        }
    }
} satisfies Actions;