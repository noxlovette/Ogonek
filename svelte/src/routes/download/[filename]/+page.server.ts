import { error } from "@sveltejs/kit";

export async function load({ params, fetch }) {
  const filename = params.filename.replace(/^"|"$/g, "");

  if (!filename.match(/^[a-zA-Z0-9-_.]+$/)) {
    throw error(400);
  }

  const response = await fetch(`/file-server/download/${filename}`);
  const data = await response.arrayBuffer();

  console.debug(data);

  return {
    body: data,
    filename,
    headers: {
      "Content-Type":
        response.headers.get("Content-Type") || "application/octet-stream",
      "Content-Disposition": response.headers.get("Content-Disposition"),
    },
  };
}
