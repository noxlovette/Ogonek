import { json } from "@sveltejs/kit";
const MOCK_CODES = [200, 401, 404, 501, 204] as const;
type MockResponseCode = (typeof MOCK_CODES)[number];

export function getMockResponseCode(url: URL): MockResponseCode {
  const code = parseInt(url.searchParams.get("mock_status") || "200", 10);
  return MOCK_CODES.includes(code as MockResponseCode)
    ? (code as MockResponseCode)
    : 200;
}

type ResponseBuilder<T> = () => T;

export function mockResponder<T>(
  url: URL,
  responses: Partial<Record<MockResponseCode, ResponseBuilder<T>>>,
) {
  const code = getMockResponseCode(url);

  switch (code) {
    case 200:
      return json(responses[200]?.() ?? null, { status: 200 });
    case 401:
      return json(responses[401]?.() ?? { errorr: "Unauthorised" }, {
        status: 401,
      });
    case 404:
      return json(responses[404]?.() ?? { error: "Not found" }, {
        status: 404,
      });
    case 204:
      return json(null, { status: 204 });
    default:
      return json(responses[501]?.() ?? { error: "Not implemented" }, {
        status: 501,
      });
  }
}
