<script lang="ts">
  import { user, mobileMenuOpen } from "$lib/stores";
  import ThemeToggler from "../interactive/ThemeToggler.svelte";
  import Clock from "../Clock.svelte";
  import { Menu } from "lucide-svelte";
</script>

<header class="w-full items-baseline">
  <div
    class="relative mx-auto flex w-full items-center justify-between px-4 py-3"
  >
    <div
      class="z-50 flex w-full items-center justify-center md:w-1/6 md:items-start md:justify-start"
    >
      <a href="/" class="font-serif text-2xl font-bold">Ogonek</a>
    </div>
    {#if $user.username}
      <button
        aria-label={$mobileMenuOpen ? "Close menu" : "Open menu"}
        class="absolute right-0 z-50 flex flex-col items-center justify-center rounded-md p-2 transition-colors hover:bg-stone-200 md:hidden dark:hover:bg-stone-700"
        onclick={() => mobileMenuOpen.toggle()}
      >
        <Menu />
      </button>
    {/if}
    {#if $user.name}
      <div class="hidden min-w-fit items-center space-x-4 md:flex">
        <Clock />
        <ThemeToggler />
      </div>
    {:else}
      <div class="hidden min-w-fit items-center space-x-4 md:flex">
        <a href="/auth/login" class="font-serif text-sm font-bold md:text-lg"
          >Login</a
        >
        <a href="/auth/signup" class="font-serif text-sm font-bold md:text-lg"
          >Signup</a
        >
      </div>
    {/if}
  </div>
</header>
