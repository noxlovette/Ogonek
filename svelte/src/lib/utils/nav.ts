export function getFormattedPath(path: string): string[] {
  // Remove leading slash and split by '/'
  const segments = path.replace(/^\//, "").split("/");

  // Skip the role prefix (t or s) if it exists
  const startIndex = segments[0] === "t" || segments[0] === "s" ? 1 : 0;

  // Map each segment to a capitalized word
  return segments.slice(startIndex).map((segment) => {
    // Handle special cases
    if (segment === "words") return "Flashcards";
    if (segment === "learn") return "Learning";

    // Default: capitalize segment
    return segment.charAt(0).toUpperCase() + segment.slice(1);
  });
}
