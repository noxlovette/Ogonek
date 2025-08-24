<script lang="ts">
  import { onMount } from "svelte";
  import { Sun, Moon, Monitor } from "lucide-svelte";
  import { notification } from "$lib/stores";

  let theme = $state("auto");

  function applyTheme(mode: string) {
    if (mode === "auto") {
      const systemPrefersDark = window.matchMedia(
        "(prefers-color-scheme: dark)",
      ).matches;
      document.documentElement.classList.toggle("dark", systemPrefersDark);
    } else {
      document.documentElement.classList.toggle("dark", mode === "dark");
    }
  }

  onMount(() => {
    const storedTheme = localStorage.getItem("theme");
    theme = storedTheme || "auto";
    applyTheme(theme);
  });

  function setTheme(newTheme: string) {
    theme = newTheme;
    notification.set({ message: theme, type: "info" });
    localStorage.setItem("theme", theme);
    applyTheme(theme);
  }
</script>

<div
  class="relative inline-flex w-max rounded-full bg-stone-200/60 p-1 dark:bg-stone-800/60"
>
  <!-- Sliding background indicator -->
  <div
    class="absolute top-1 h-8 w-10 rounded-full bg-white shadow-sm transition-transform duration-200 ease-out dark:bg-stone-600"
    style="transform: translateX({theme === 'light'
      ? '0px'
      : theme === 'dark'
        ? '40px'
        : '80px'})"
  ></div>

  <!-- Theme buttons -->
  <button
    onclick={() => setTheme("light")}
    class="relative z-10 flex h-8 w-10 items-center justify-center rounded-md transition-colors duration-150"
    class:text-accent={theme === "light"}
    class:text-stone-600={theme !== "light"}
    aria-label="Light theme"
  >
    <Sun class="h-4 w-4" />
  </button>

  <button
    onclick={() => setTheme("dark")}
    class="relative z-10 flex h-8 w-10 items-center justify-center rounded-md transition-colors duration-150"
    class:text-accent={theme === "dark"}
    class:text-stone-600={theme !== "dark"}
    aria-label="Dark theme"
  >
    <Moon class="h-4 w-4" />
  </button>

  <button
    onclick={() => setTheme("auto")}
    class="relative z-10 flex h-8 w-10 items-center justify-center rounded-md transition-colors duration-150"
    class:text-accent={theme === "auto"}
    class:text-stone-600={theme !== "auto"}
    aria-label="Auto theme"
  >
    <Monitor class="h-4 w-4" />
  </button>
</div>
