<script lang="ts">
  import { user } from "$lib/stores";

  let href = $user.role === "teacher" ? "/t/dashboard" : "/s/dashboard";
</script>

<div
  class="flex w-full flex-col items-center space-y-4 p-4 text-center md:p-6 md:text-left lg:p-8"
>
  <h1
    class="text-[3rem] leading-[3rem] font-bold md:leading-[4rem] xl:text-[4rem]"
  >
    Welcome to Ogonek
  </h1>
  <h2 class="text-xl">
    See your homework, revisit previous lessons. One place for your English
  </h2>
  <div class="mt-8 space-x-2">
    <a
      href="/auth/login"
      class="bg-cacao-600 hover:bg-cacao-100 mt-5 w-20 rounded-lg border-2 border-stone-200 px-3 py-2 text-center text-sm text-stone-50 transition-colors hover:text-stone-800 md:px-4 md:py-2 md:text-base dark:border-stone-700 dark:bg-stone-900 dark:hover:bg-stone-600 dark:hover:text-stone-50"
      >Log In</a
    >
  </div>
</div>
