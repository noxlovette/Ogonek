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
  accessToken: {
    token: string;
    expiresAt: string;
  };
  refreshToken: {
    token: string;
    expiresAt: string;
  };
}

export interface RefreshResponse {
  accessToken: {
    token: string;
    expiresAt: string;
  };
}

export type EmptyResponse = Record<string, never>;

export interface UploadResponse {
  filePath: string;
}

export interface URLResponse {
  url: string;
}
