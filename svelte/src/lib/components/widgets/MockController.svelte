<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import ThemeToggler from "../UI/interactive/ThemeToggler.svelte";
  import { UniButton } from "../UI";
  import { page } from "$app/state";

  let currentLocale = "en";

  const languages = [
    { code: "en", name: "EN", flag: "🇺🇸" },
    { code: "de", name: "DE", flag: "🇩🇪" },
    { code: "ru", name: "RU", flag: "🇷🇺" },
    { code: "fr", name: "FR", flag: "🇫🇷" },
    { code: "bo", name: "BO", flag: "🎩" },
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

<div class="fixed right-6 bottom-6 z-50">
  <div
    class="ring-default bg-default absolute right-0 bottom-14 min-w-32 overflow-hidden rounded-xl"
    in:fade
  >
    {#each languages as lang, key}
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
    <ThemeToggler></ThemeToggler>
    <UniButton
      content="Teacher"
      href={`/t/${page.url.pathname.split("/").slice(2).join("/")}`}
      iconOnly={false}
    ></UniButton>
    <UniButton
      content="Student"
      iconOnly={false}
      href={`/s/${page.url.pathname.split("/").slice(2).join("/")}`}
    ></UniButton>
  </div>
</div>
