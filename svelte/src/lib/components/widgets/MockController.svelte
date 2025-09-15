<script lang="ts">
  import { onMount } from "svelte";
  import { UniButton } from "../UI";
  import { ArrowBigUp } from "lucide-svelte";
  import { fade } from "svelte/transition";
  import ThemeToggler from "../UI/interactive/ThemeToggler.svelte";

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

  onMount(() => {
    const savedLocale = getCookie("PARAGLIDE_LOCALE");
    if (savedLocale) {
      currentLocale = savedLocale;
    }
  });
</script>

<div class="fixed right-6 bottom-6 z-50">
  <UniButton Icon={ArrowBigUp} onclick={toggleDropdown} />

  {#if isOpen}
    <div
      class="absolute right-0 bottom-14 min-w-32 overflow-hidden rounded-lg bg-white shadow-xl ring-1 ring-stone-200 transition-all duration-200"
      in:fade
    >
      {#each languages as lang}
        <button
          on:click={() => selectLanguage(lang.code)}
          class="flex w-full items-center gap-3 px-4 py-3 text-left text-sm transition-colors hover:bg-stone-50"
          class:bg-stone-50={currentLocale === lang.code}
          class:text-stone-800={currentLocale === lang.code}
          class:font-medium={currentLocale === lang.code}
        >
          <span class="text-base">{lang.flag}</span>
          <span>{lang.name}</span>
          {#if currentLocale === lang.code}
            <div class="ml-auto h-2 w-2 rounded-full bg-stone-600"></div>
          {/if}
        </button>
      {/each}
    </div>
  {/if}
  <ThemeToggler></ThemeToggler>
</div>
