import type { SortBy, SortOrder } from "$lib/types";
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

export const currentPage = createPageStore();
export const searchTerm = writable<string>();
export const assigneeStore = writable<string>();
export const pageSize = writable<number>();
export const completedStore = writable(false);
export const sortBy = writable<SortBy>("created_at");
export const sortOrder = writable<SortOrder>("desc");

export const isLoading = createToggleStore();
export const mobileMenuOpen = createToggleStore();
