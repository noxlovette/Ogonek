import { writable } from "svelte/store";

// Draggable Stores
interface Position {
  x: number;
  y: number;
}

function createDraggableStore() {
  const defaultPos = {
    x: typeof window !== "undefined" ? window.innerWidth * 0.5 : 400,
    y: 0,
  };

  const { subscribe, set, update } = writable<Position>(defaultPos);

  return {
    subscribe,
    setPosition: (pos: Position) => {
      set(pos);
    },
    updatePosition: (delta: { dx: number; dy: number }) => {
      update((pos) => {
        const newPos = { x: pos.x + delta.dx, y: pos.y + delta.dy };
        return newPos;
      });
    },
  };
}

export const panelPosition = createDraggableStore();
