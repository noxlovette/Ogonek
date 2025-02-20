import { error } from "@sveltejs/kit";

export async function load({ params, fetch }) {
  const filename = params.filename.replace(/^"|"$/g, "");

  if (!filename.match(/^[a-zA-Z0-9-_.]+$/)) {
    throw error(400);
  }

  const response = fetch(`/file-server/download/${filename}`);

  return {
    body: response.then((res) => res.arrayBuffer()), // Promise stays unresolved initially
    filename,
    headers: response.then((res) => ({
      "Content-Type":
        res.headers.get("Content-Type") || "application/octet-stream",
      "Content-Disposition": res.headers.get("Content-Disposition"),
    })),
  };
}
