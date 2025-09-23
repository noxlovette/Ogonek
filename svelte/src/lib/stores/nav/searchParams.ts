import { writable } from "svelte/store";

// URL Params
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
