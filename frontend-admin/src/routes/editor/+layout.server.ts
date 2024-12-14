import type { LayoutServerLoad } from "./$types";

const VITE_API_URL = "http://backend:8000";

let cache = {
  categories: null,
  tags: null,
  concepts: null,
  articles: null,
};
let lastFetch = 0;
const CACHE_DURATION = 60 * 1000;

export const load: LayoutServerLoad = async () => {
  const now = Date.now();

  // Check if cached data is still valid
  if (now - lastFetch < CACHE_DURATION && cache.categories) {
    return cache;
  }

  // Fetch new data
  const [categories, tags, concepts, articles] = await Promise.all([
    fetch(`${VITE_API_URL}/api/categories/`).then((res) => res.json()),
    fetch(`${VITE_API_URL}/api/tags/`).then((res) => res.json()),
    fetch(`${VITE_API_URL}/api/concepts/`).then((res) => res.json()),
    fetch(`${VITE_API_URL}/api/articles/`).then((res) => res.json()),
  ]);

  // Update the cache
  cache = { categories, tags, concepts, articles };
  lastFetch = now;

  return cache;
};