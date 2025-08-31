<script lang="ts">
  import { onMount } from "svelte";

  let currentLocale = "en";
  let isOpen = false;

  const languages = [
    { code: "en", name: "EN", flag: "🇺🇸" },
    { code: "de", name: "DE", flag: "🇩🇪" },
    { code: "ru", name: "RU", flag: "🇷🇺" },
    { code: "fr", name: "FR", flag: "🇫🇷" },
  ];

  function getCookie(name: string): string | null {
    const value = `; ${document.cookie}`;
    const parts = value.split(`; ${name}=`);
    if (parts.length === 2) return parts.pop()!.split(";").shift() || null;
    return null;
  }

  function setLanguageCookie(locale: string) {
    document.cookie = `PARAGLIDE_LOCALE=${locale}; Path=/; Max-Age=31536000; SameSite=Lax`;
    window.location.reload();
  }

  function toggleDropdown() {
    isOpen = !isOpen;
  }

  function selectLanguage(locale: string) {
    setLanguageCookie(locale);
    isOpen = false;
  }

  // Close dropdown when clicking outside
  function handleClickOutside(event: Event) {
    if (!event.target?.closest(".lang-switcher")) {
      isOpen = false;
    }
  }

  onMount(() => {
    const savedLocale = getCookie("PARAGLIDE_LOCALE");
    if (savedLocale) {
      currentLocale = savedLocale;
    }

    document.addEventListener("click", handleClickOutside);
    return () => document.removeEventListener("click", handleClickOutside);
  });

  $: currentLang =
    languages.find((lang) => lang.code === currentLocale) || languages[0];
</script>

<div class="lang-switcher fixed right-6 bottom-6 z-50">
  <!-- Main clickable arrow button -->
  <button
    on:click={toggleDropdown}
    class="group flex h-12 w-12 items-center justify-center rounded-full bg-stone-900 text-white shadow-lg transition-all duration-200 hover:scale-110 hover:bg-stone-800 active:scale-95"
    class:rotate-180={isOpen}
  >
    <svg
      class="h-5 w-5 transition-transform duration-200"
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M5 15l7-7 7 7"
      />
    </svg>
  </button>

  <!-- Dropdown menu -->
  {#if isOpen}
    <div
      class="absolute right-0 bottom-14 min-w-32 overflow-hidden rounded-lg bg-white shadow-xl ring-1 ring-stone-200 transition-all duration-200"
      class:animate-in={isOpen}
    >
      {#each languages as lang}
        <button
          on:click={() => selectLanguage(lang.code)}
          class="flex w-full items-center gap-3 px-4 py-3 text-left text-sm transition-colors hover:bg-stone-50"
          class:bg-amber-50={currentLocale === lang.code}
          class:text-amber-800={currentLocale === lang.code}
          class:font-medium={currentLocale === lang.code}
        >
          <span class="text-base">{lang.flag}</span>
          <span>{lang.name}</span>
          {#if currentLocale === lang.code}
            <div class="ml-auto h-2 w-2 rounded-full bg-amber-600"></div>
          {/if}
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  @keyframes animate-in {
    from {
      opacity: 0;
      transform: translateY(8px) scale(0.95);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  .animate-in {
    animation: animate-in 150ms ease-out;
  }
</style>
