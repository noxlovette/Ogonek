import { json } from '@sveltejs/kit';

// Generated mock for POST /api/v1/notifications/request
// Operation: request_hw




export async function POST({ request, params, url }: RequestEvent) {
  // Mock response selector - customize this logic
  const mockResponse = url.searchParams.get('mock_status') || '204';
  
  
  
  
  
  // Return mock based on requested status
  switch (parseInt(mockResponse)) {
    case 204:
      return new Response(null, { status: 204 });

    case 401:
      return json(null, { status: 401 });
    default:
      return json({ error: "Not implemented" }, { status: 501 });
  }
}