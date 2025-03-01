// src/lib/types/api.ts - TYPES ONLY
export interface ApiErrorResponse {
  error: {
    message: string;
    code: number;
  };
}

export type ApiResult<T> =
  | { success: true; data: T }
  | { success: false; status: number; message: string };

export interface SignupResponse {
  id: string;
}

export interface NewResponse {
  id: string;
}

export interface AuthResponse {
  accessToken: string;
  refreshToken?: string;
}

export type EmptyResponse = Record<string, never>;

export interface UploadResponse {
  data: string;
  success: boolean;
}
