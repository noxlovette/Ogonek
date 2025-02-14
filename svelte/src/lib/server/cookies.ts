type CookieResponse = Response & {
  headers: Headers & {
    get(name: "set-cookie"): string | null;
    getSetCookie(): string[];
  };
};

export const setCookiesFromResponse = (
  originalResponse: Response,
  cookieResponse: CookieResponse,
): Response => {
  const newResponse = new Response(originalResponse.body, originalResponse);
  const setCookieHeaders = cookieResponse.headers.getSetCookie();

  setCookieHeaders.forEach((cookie) => {
    newResponse.headers.append("set-cookie", cookie);
  });

  return newResponse;
};

export interface CookieOptions {
  path: string;
  httpOnly?: boolean;
  secure?: boolean;
  sameSite?: "lax" | "strict" | "none";
  domain?: string;
  maxAge?: number;
}

export const parseCookieOptions = (opts: string[]): CookieOptions => {
  const options: CookieOptions = { path: "/" }; // Ensure path is always defined

  opts.forEach((opt) => {
    const [key, val] = opt.trim().split("=");
    const normalizedKey = key.toLowerCase().replace(/-/g, "");

    switch (normalizedKey) {
      case "path":
        options.path = val || "/";
        break;
      case "httponly":
        options.httpOnly = true;
        break;
      case "secure":
        options.secure = true;
        break;
      case "samesite":
        options.sameSite = val as "lax" | "strict" | "none";
        break;
      case "domain":
        options.domain = val;
        break;
      case "maxage":
      case "max-age":
        options.maxAge = val ? parseInt(val) : undefined;
        break;
    }
  });

  return options;
};
