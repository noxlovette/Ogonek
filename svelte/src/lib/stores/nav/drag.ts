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
export const panelSide = writable<"left" | "right">("left");
export function setPanelSideFromCalendarClick(dayOfWeek: number) {
  // 0-3 = left side (Sun, Mon, Tue, Wed) -> panel goes right
  // 4-6 = right side (Thu, Fri, Sat) -> panel goes left
  const side = dayOfWeek <= 3 ? "right" : "left";
  panelSide.set(side);
}
