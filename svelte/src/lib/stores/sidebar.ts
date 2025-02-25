// src/lib/stores/sidebar.ts
import { writable } from "svelte/store";

const createSidebarStore = () => {
  const { subscribe, set, update } = writable<boolean>(false);

  return {
    subscribe,
    toggle: () => {
      update((state) => {
        const newState = !state;
        return newState;
      });
    },
    collapse: () => {
      set(true);
    },
    expand: () => {
      set(false);
    },
    setCollapsed: (value: boolean) => {
      set(value);
    },
  };
};

export const sidebar = createSidebarStore();
