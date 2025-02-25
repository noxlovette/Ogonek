<script lang="ts">
  import { getGreeting } from "$lib/utils";
  import { user } from "$lib/stores";
  import Search from "../search/SearchHeader.svelte";
  import ThemeToggler from "../interactive/ThemeToggler.svelte";
  const greeting = getGreeting();
  let href = $user.role === "teacher" ? "/t/dashboard" : "/s/dashboard";
</script>

<header
  class="ring-milk-200 dark:ring-milk-900 my-2 w-11/12 items-baseline rounded-lg shadow-md ring md:w-full"
>
  <div
    class="mx-auto flex w-full max-w-7xl items-center justify-between px-4 py-3"
  >
    <div class="flex w-full md:w-1/6">
      <a {href} class="font-serif text-2xl font-bold">Firelight</a>
    </div>

    {#if $user.name}
      <Search />

      <div class="hidden w-1/6 min-w-fit items-center space-x-4 md:flex">
        <p class="flex-shrink text-sm">
          {$user.name}, good {greeting}
        </p>
        <ThemeToggler />
      </div>
    {:else}
      <div class="flex min-w-fit items-center space-x-2 md:w-1/6 md:space-x-4">
        <a href="/auth/login" class="font-serif text-sm font-bold md:text-lg"
          >Login</a
        >
        <a href="/auth/signup" class="font-serif text-sm font-bold md:text-lg"
          >Signup</a
        >
        <ThemeToggler />
      </div>
    {/if}
  </div>
</header>
