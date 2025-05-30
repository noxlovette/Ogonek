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

  function toggleTheme() {
    theme = theme === "light" ? "dark" : theme === "dark" ? "auto" : "light";
    notification.set({ message: theme, type: "info" });

    localStorage.setItem("theme", theme);
    applyTheme(theme);
  }
</script>

<button
  onclick={toggleTheme}
  class="ring-default rounded-t-lg bg-stone-50/30 p-2 transition-colors duration-150 hover:bg-stone-100
         dark:bg-stone-900/30
          dark:hover:bg-stone-700"
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
