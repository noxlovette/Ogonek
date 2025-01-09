// import {BACKEND_URL} from '$env/static/private';
import {env} from '$env/dynamic/private';

export class APIClient {
    private static instance: APIClient;
    private readonly baseURL: string;
    private readonly defaultHeaders: { 'Content-Type': string; 'X-API-Key': string; 'Authorization'?: string };

    private constructor() {
        this.baseURL = env.BACKEND_URL; // ou $env/static/private si tu préfères
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

    async request<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
        const headers = new Headers({
            ...this.defaultHeaders,
            ...options.headers,
        });

        const response = await fetch(`${this.baseURL}${endpoint}`, {
            ...options,
            headers,
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        return response;
    }

    setAuthToken(token: string) {
        this.defaultHeaders['Authorization'] = `Bearer ${token}`;
    }
}
