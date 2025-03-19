<script lang="ts">
  import { user } from "$lib/stores";
  import { page } from "$app/state";
  import ThemeToggler from "../interactive/ThemeToggler.svelte";
  import Clock from "../Clock.svelte";

  let href = page.params.role === "t" ? "/t/dashboard" : "/s/dashboard";
</script>

<header class="w-full items-baseline">
  <div class="mx-auto flex w-full items-center justify-between px-4 py-3">
    <div class="z-50 flex w-full items-center justify-center md:w-1/6">
      <a {href} class="font-serif text-2xl font-bold">Ogonek</a>
    </div>

    {#if $user.name}
      <div class="hidden min-w-fit items-center space-x-4 md:flex">
        <Clock />
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
