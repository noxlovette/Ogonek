import { createMockTaskWithFiles } from "$lib/server/mock";
import { json, type RequestEvent } from "@sveltejs/kit";

// Generated mock for GET /api/v1/tasks/{id}
// Operation: One Task

export async function GET({ request, params, url }: RequestEvent) {
  // Mock response selector - customize this logic
  const mockResponse = url.searchParams.get("mock_status") || "200";

  // Path params: id

  // Return mock based on requested status
  switch (parseInt(mockResponse)) {
    case 200:
      return json(createMockTaskWithFiles({ id: params.id }), { status: 200 });

    case 401:
      return json(null, { status: 401 });

    case 404:
      return json(null, { status: 404 });
    default:
      return json({ error: "Not implemented" }, { status: 501 });
  }
}
