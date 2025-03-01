// src/lib/api.ts - IMPLEMENTATION
import type { ApiErrorResponse, ApiResult } from "$lib/types";

// Helper to handle API responses with proper typing
export async function handleApiResponse<T>(
  response: Response,
): Promise<ApiResult<T>> {
  if (!response.ok) {
    const errorData = (await response.json()) as ApiErrorResponse;
    return {
      success: false,
      status: response.status,
      message: errorData.error.message,
    };
  }

  // For 204 No Content
  if (response.status === 204) {
    return { success: true, data: {} as T };
  }

  // For responses with content
  const data = (await response.json()) as T;
  return { success: true, data };
}

export function isSuccessResponse<T>(
  response: ApiResult<T>,
): response is { success: true; data: T } {
  return response.success === true;
}
