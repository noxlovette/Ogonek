<script lang="ts">
  import { onMount } from "svelte";
  import { Merger, UniButton } from "../";
  import { ChevronDown } from "lucide-svelte";
  import { fade } from "svelte/transition";

  let currentLocale = "en";
  let isOpen = false;

  const languages = [
    { code: "en", name: "English", flag: "🇺🇸" },
    { code: "de", name: "Deutsch", flag: "🇩🇪" },
    { code: "ru", name: "Русский", flag: "🇷🇺" },
    { code: "fr", name: "Français", flag: "🇫🇷" },
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

  $: selectedLanguage =
    languages.find((lang) => lang.code === currentLocale) || languages[0];

  onMount(() => {
    const savedLocale = getCookie("PARAGLIDE_LOCALE");
    if (savedLocale) {
      currentLocale = savedLocale;
    }
  });
</script>

<div class="relative">
  <Merger>
    <UniButton
      type="button"
      onclick={toggleDropdown}
      iconOnly={false}
      content={selectLanguage.name}
      variant="primary"
    ></UniButton>
  </Merger>
  {#if isOpen}
    <div
      class="bg-default ring-default absolute top-full left-0 z-50 mt-1 w-full overflow-hidden rounded-2xl shadow-xl"
      in:fade={{ duration: 200 }}
    >
      {#each languages as lang}
        <button
          onclick={() => selectLanguage(lang.code)}
          class="flex w-full items-center gap-3 px-4 py-3 text-left text-sm transition-colors hover:bg-stone-100 dark:hover:bg-stone-700"
          class:bg-stone-50={currentLocale === lang.code}
          class:dark:bg-stone-700={currentLocale === lang.code}
          class:text-stone-800={currentLocale === lang.code}
          class:font-medium={currentLocale === lang.code}
        >
          <span class="text-base">{lang.flag}</span>
          <span>{lang.name}</span>
          {#if currentLocale === lang.code}
            <div class="bg-accent ml-auto h-2 w-2 rounded-full"></div>
          {/if}
        </button>
      {/each}
    </div>
  {/if}
</div>
