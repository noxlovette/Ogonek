import { json } from '@sveltejs/kit';

// Generated mock for POST /api/v1/decks{id}/duplicate
// Operation: Duplicates a deck, returns new id




export async function POST({ request, params, url }: RequestEvent) {
  // Mock response selector - customize this logic
  const mockResponse = url.searchParams.get('mock_status') || '200';
  
  
  
  
  
  // Return mock based on requested status
  switch (parseInt(mockResponse)) {
    case 200:
      return json(null, { status: 200 });

    case 400:
      return json(null, { status: 400 });

    case 401:
      return json(null, { status: 401 });
    default:
      return json({ error: "Not implemented" }, { status: 501 });
  }
}