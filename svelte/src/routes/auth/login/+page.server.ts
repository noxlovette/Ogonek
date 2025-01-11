import { fail, type Actions } from '@sveltejs/kit';

export const actions: Actions = {
    default: async ({ request, fetch, locals }) => {
        const data = await request.formData();
        const username = data.get('username') as string;
        const pass = data.get('password') as string;

        try {
            const response = await fetch(`/axum/auth/signin`, {
                method: 'POST',
                body: JSON.stringify({
                    username,
                    pass,
                })
            });

            if (!response.ok) {
                throw new Error('Login failed');
            }

            const { accessToken } = await response.json();
            locals.accessToken = accessToken;

            console.log("locals", locals);



            return {
                success: true,
                message: 'Login successful',
                accessToken,
            };

        } catch (error) {
            console.error('Signin error:', error);
            return fail(400, {
                success: false,
                message: error instanceof Error ? error.message : 'Login failed'
            });
        }
    }
};