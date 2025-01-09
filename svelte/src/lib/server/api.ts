import { env } from '$env/dynamic/private';
import type { Cookies } from '@sveltejs/kit';

interface RequestOptions extends RequestInit {
  skipAuth?: boolean;
}

export class APIClient {
  private static instance: APIClient;
  private readonly baseURL: string;
  private readonly defaultHeaders: Record<string, string>;
  private cookies?: Cookies;

  private constructor() {
    this.baseURL = env.BACKEND_URL;
    this.defaultHeaders = {
      'Content-Type': 'application/json',
      'X-API-Key': env.API_KEY_AXUM,
    };
  }

  static getInstance(): APIClient {
    if (!APIClient.instance) {
      APIClient.instance = new APIClient();
    }
    return APIClient.instance;
  }

  setCookies(cookies: Cookies): void {
    this.cookies = cookies;
  }

  setAuthToken(token: string): void {
    this.defaultHeaders['Authorization'] = `Bearer ${token}`;
  }

  setRefreshToken(token: string): void {
    if (!this.cookies) {
      throw new Error('Cookies not initialized. Call setCookies first.');
    }

    this.cookies.set('refresh_token', token, {
      path: '/',
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production',
      sameSite: 'strict',
      maxAge: 60 * 60 * 24 * 7, // 7 days
    });
  }

  async request<T>(
    endpoint: string, 
    options: RequestOptions = {}
  ): Promise<T> {
    const { skipAuth = false, ...fetchOptions } = options;
    
    const headers = new Headers({
      ...this.defaultHeaders,
      ...fetchOptions.headers,
    });

    // Si on skip l'auth, on retire le header Authorization
    if (skipAuth && headers.has('Authorization')) {
      headers.delete('Authorization');
    }

    const response = await fetch(`${this.baseURL}${endpoint}`, {
      ...fetchOptions,
      headers,
      credentials: 'include', // Important pour les cookies
    });

    if (!response.ok) {
      if (response.status === 401 && !skipAuth) {
        // Tentative de refresh du token
        try {
          const newTokens = await this.refreshTokens();
          this.setAuthToken(newTokens.accessToken);
          this.setRefreshToken(newTokens.refreshToken);
          
          // On retry la requête originale
          return this.request(endpoint, options);
        } catch (error) {
          throw new Error('Session expired');
        }
      }
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    // Handle 204 No Content responses
    if (response.status === 204) {
      return null as T;
    }
    
    // Only try to parse JSON if there's content
    const contentType = response.headers.get('content-type');
    if (contentType && contentType.includes('application/json')) {
      return response.json();
    }
    
    return null as T;
  }

  private async refreshTokens(): Promise<{ accessToken: string; refreshToken: string }> {
    const response = await this.request<{ accessToken: string; refreshToken: string }>(
      '/auth/refresh',
      { skipAuth: true }
    );
    return response;
  }

  // Helpers pour les méthodes HTTP communes
  async get<T>(endpoint: string, options: RequestOptions = {}): Promise<T> {
    return this.request<T>(endpoint, { ...options, method: 'GET' });
  }

  async post<T>(endpoint: string, data?: unknown, options: RequestOptions = {}): Promise<T> {
    return this.request<T>(endpoint, {
      ...options,
      method: 'POST',
      body: data ? JSON.stringify(data) : undefined,
    });
  }
}