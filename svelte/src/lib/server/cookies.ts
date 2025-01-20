type CookieResponse = Response & {
	headers: Headers & {
		get(name: 'set-cookie'): string | null;
		getSetCookie(): string[];
	};
};

export const setCookiesFromResponse = (
	originalResponse: Response,
	cookieResponse: CookieResponse
): Response => {
	const newResponse = new Response(originalResponse.body, originalResponse);
	const setCookieHeaders = cookieResponse.headers.getSetCookie();

	setCookieHeaders.forEach((cookie) => {
		newResponse.headers.append('set-cookie', cookie);
	});

	return newResponse;
};
