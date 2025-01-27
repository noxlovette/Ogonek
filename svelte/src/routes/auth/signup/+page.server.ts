import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { env } from '$env/dynamic/private';
import { z } from 'zod';

const UserSchema = z.object({
	username: z
		.string()
		.min(3, 'Username must be at least 3 characters')
		.max(20, 'Username must be less than 20 characters')
		.regex(
			/^[a-zA-Z0-9_-]+$/,
			'Username can only contain letters, numbers, underscores, and dashes'
		),
	password: z
		.string()
		.min(8, 'Password must be at least 8 characters')
		.regex(/[A-Z]/, 'Password must contain at least one uppercase letter')
		.regex(/[a-z]/, 'Password must contain at least one lowercase letter')
		.regex(/[0-9]/, 'Password must contain at least one number')
		.regex(/[^A-Za-z0-9]/, 'Password must contain at least one special character'),
	email: z.string().email('Invalid email format').optional(),
	role: z
		.enum(['student', 'teacher', 'admin'], {
			errorMap: () => ({ message: 'Invalid role selected' })
		})
		.optional(),
	name: z
		.string()
		.min(2, 'Name must be at least 2 characters')
		.max(50, 'Name must be less than 50 characters')
		.optional()
});

const turnstileVerify = async (turnstileToken: string) => {
	const response = await fetch('https://challenges.cloudflare.com/turnstile/v0/siteverify', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/x-www-form-urlencoded'
		},
		body: new URLSearchParams({
			secret: env.CLOUDFLARE_SECRET,
			response: turnstileToken
		})
	});

	if (!response.ok) {
		throw new Error(`Turnstile verification failed: ${response.status}`);
	}

	return response.json();
};

type ValidationResult = {
	success: boolean;
	data?: z.infer<typeof UserSchema>;
	errors?: Array<{ field: string; message: string }>;
};

const validateInput = (formData: FormData): ValidationResult => {
	try {
		const data = {
			username: formData.get('username'),
			password: formData.get('password'),
			email: formData.get('email'),
			role: formData.get('role'),
			name: formData.get('name')
		};

		const result = UserSchema.safeParse(data);

		if (!result.success) {
			return {
				success: false,
				errors: result.error.issues.map((issue) => ({
					field: issue.path[0] as string,
					message: issue.message
				}))
			};
		}

		return {
			success: true,
			data: result.data
		};
	} catch (error) {
		return {
			success: false,
			errors: [{ field: 'general', message: 'Invalid form data' }]
		};
	}
};

export const actions: Actions = {
	default: async ({ request, url, fetch }) => {
		const data = await request.formData();
		const turnstileToken = data.get('cf-turnstile-response');
		const invite_token = url.searchParams.get('invite');

		// Validate turnstile first
		if (!turnstileToken) {
			return fail(400, {
				message: 'Please complete the CAPTCHA verification'
			});
		}

		const turnstileResponse = await turnstileVerify(turnstileToken as string);
		if (!turnstileResponse.success) {
			return fail(400, {
				message: 'CAPTCHA verification failed. Please try again'
			});
		}

		// Validate user input
		const validationResult = validateInput(data);
		if (!validationResult.success) {
			return fail(400, {
				message: 'Validation failed',
				errors: validationResult.errors
			});
		}

		try {
			const response = await fetch('/axum/auth/signup', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(validationResult.data)
			});

			if (!response.ok) {
				const { error } = await response.json();
				return fail(response.status, {
					message: error || 'Failed to create account'
				});
			}

			// Handle invite token if present
			if (invite_token) {
				const { id: student_id } = await response.json();
				const invite = await fetch('/axum/auth/bind', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					body: JSON.stringify({
						invite_token,
						student_id
					})
				});

				if (!invite.ok) {
					const { error } = await invite.json();
					return fail(invite.status, {
						message: error || 'Failed to bind invite'
					});
				}
			}

			return redirect(302, '/auth/login');
		} catch (error) {
			console.error('Registration error:', error);
			return fail(500, {
				message: 'An unexpected error occurred. Please try again later.'
			});
		}
	}
} satisfies Actions;
