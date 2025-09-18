import { writable } from "svelte/store";

function createPageStore() {
  const { subscribe, set, update } = writable(1);

  return {
    subscribe,
    set,
    increase: () => update((n) => n + 1),
    decrease: () => update((n) => (n > 1 ? n - 1 : 1)),
    reset: () => set(1),
  };
}

function createSearchTermStore() {
  const { subscribe, set, update } = writable("");

  return {
    subscribe,
    update,
    set,
    reset: () => set(""),
  };
}

function createAssigneeStore() {
  const { subscribe, set, update } = writable("");

  return {
    subscribe,
    update,
    set,
    reset: () => set(""),
  };
}

function createPageSize() {
  const { subscribe, set, update } = writable(50);

  return {
    subscribe,
    set,
    update,
    reset: () => set(50),
  };
}

export const currentPage = createPageStore();
export const searchTerm = createSearchTermStore();
export const assigneeStore = createAssigneeStore();
export const pageSize = createPageSize();
export const completedStore = createCompleted();

function createCompleted() {
  const { subscribe, set } = writable(false);

  return {
    subscribe,
    true: () => set(true),
    false: () => set(false),
    toggle: () => {
      let currentValue;
      subscribe((value) => {
        currentValue = value;
      })();
      set(!currentValue);
    },
  };
}

function createToggleStore() {
  const { subscribe, set } = writable(false);

  return {
    subscribe,
    true: () => set(true),
    false: () => set(false),
    toggle: () => {
      let currentValue;
      subscribe((value) => {
        currentValue = value;
      })();
      set(!currentValue);
    },
  };
}

export const isLoading = createToggleStore();
export const mobileMenuOpen = createToggleStore();

// lib/stores/draggable.ts

interface Position {
  x: number;
  y: number;
}

function createDraggableStore() {
  // Load from localStorage or default to right side
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
