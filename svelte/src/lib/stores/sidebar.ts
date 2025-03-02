// src/lib/stores/sidebar.ts
import { browser } from "$app/environment";
import { writable } from "svelte/store";

const createSidebarStore = (key: string) => {
  const initialValue = browser
    ? JSON.parse(localStorage.getItem(key) || "false")
    : false;

  const { subscribe, set, update } = writable<boolean>(initialValue);

  // Subscribe to changes and update localStorage
  if (browser) {
    subscribe((value) => {
      localStorage.setItem(key, JSON.stringify(value));
    });
  }

  return {
    subscribe,
    toggle: () => {
      update((state) => !state);
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

export const sidebar = createSidebarStore("sidebar_collapsed");
export const rightbar = createSidebarStore("rightbar_collapsed");
