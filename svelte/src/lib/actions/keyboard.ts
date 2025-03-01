export function keyEscape(node: HTMLElement, callback: () => void) {
  const handleKeydown = (event: KeyboardEvent) => {
    if (event.key === "Escape") {
      event.preventDefault();
      callback();
    }
  };

  window.addEventListener("keydown", handleKeydown);

  return {
    destroy() {
      window.removeEventListener("keydown", handleKeydown);
    },
  };
}
