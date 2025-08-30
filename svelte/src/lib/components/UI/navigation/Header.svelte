<script lang="ts">
  import { page } from "$app/state";
  import { m } from "$lib/paraglide/messages";
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

    {#if page.url.href == "/auth"}
      <div class="hidden min-w-fit items-center space-x-4 md:flex">
        <a href="/auth/login" class="font-serif text-sm font-bold md:text-lg"
          >{m.logIn()}</a
        >
        <a href="/auth/signup" class="font-serif text-sm font-bold md:text-lg"
          >{m.signUp()}</a
        >
      </div>
    {/if}
  </div>
</header>
