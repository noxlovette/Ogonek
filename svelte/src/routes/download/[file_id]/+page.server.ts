export async function load({ params, fetch }) {
  const id = params.file_id;
  const response = fetch(`/axum/file/${id}`);

  return {
    id,
    body: response.then((res) => res.arrayBuffer()),
    headers: response.then((res) => ({
      "Content-Type":
        res.headers.get("Content-Type") || "application/octet-stream",
      "Content-Disposition": res.headers.get("Content-Disposition"),
    })),
  };
}
