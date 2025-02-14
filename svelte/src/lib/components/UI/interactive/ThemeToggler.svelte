<script lang="ts">
  import { onMount } from "svelte";
  import { Sun, Moon, Monitor } from "lucide-svelte";

  // Theme state: 'light', 'dark', or 'auto'
  let theme = $state("auto");

  // Function to apply theme based on user selection or system preference
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

  // Load theme from localStorage or system preference
  onMount(() => {
    const storedTheme = localStorage.getItem("theme");
    theme = storedTheme || "auto";
    applyTheme(theme);
  });

  // Cycle between light → dark → auto
  function toggleTheme() {
    theme = theme === "light" ? "dark" : theme === "dark" ? "auto" : "light";
    localStorage.setItem("theme", theme);
    applyTheme(theme);
  }
</script>

<button
  onclick={toggleTheme}
  class="bg-milk-100 dark:bg-milk-800 hover:bg-milk-200 dark:hover:bg-milk-700
         rounded-lg p-2
         transition-colors duration-200"
  aria-label="Toggle theme"
>
  {#if theme === "light"}
    <Sun class="text-cacao-500 h-5 w-5" />
  {:else if theme === "dark"}
    <Moon class="text-cacao-500 h-5 w-5" />
  {:else}
    <Monitor class="text-cacao-500 h-5 w-5" />
  {/if}
</button>
