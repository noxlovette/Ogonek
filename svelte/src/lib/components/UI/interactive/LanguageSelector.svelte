<script lang="ts">
  import { onMount } from "svelte";
  import { Callout } from "$lib/components/typography";

  let currentLocale = "ru";

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

  function selectLanguage(locale: string) {
    setLanguageCookie(locale);
  }

  onMount(() => {
    const savedLocale = getCookie("PARAGLIDE_LOCALE");
    if (savedLocale) {
      currentLocale = savedLocale;
    }
  });
</script>

<div class="grid gap-2">
  {#each languages as lang}
    <button
      onclick={() => selectLanguage(lang.code)}
      class="hover-default ring-default flex w-full items-center gap-2 rounded-2xl bg-stone-100 p-2 dark:bg-stone-950"
      class:bg-stone-50={currentLocale === lang.code}
      class:dark:bg-stone-700={currentLocale === lang.code}
      class:text-stone-800={currentLocale === lang.code}
      class:font-medium={currentLocale === lang.code}
    >
      <span class="text-base">{lang.flag}</span>
      <Callout>
        {lang.name}
      </Callout>
      {#if currentLocale === lang.code}
        <div class="bg-accent ml-auto h-2 w-2 rounded-full"></div>
      {/if}
    </button>
  {/each}
</div>
