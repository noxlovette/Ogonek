<script lang="ts">
  import { page } from "$app/stores";
</script>

<div
  class="flex w-full flex-col items-center justify-center rounded-lg text-center"
>
  <h1 class="text-xl font-bold">Error {$page.status}: {$page.error.message}</h1>
  <a href="https://t.me/noxlovette" class="rounded text-center"
    >If you think you are not supposed to see this, please complain by clicking
    on this message.</a
  >
  <a
    href="/"
    class="bg-cacao-400 hover:bg-cacao-500 mt-4 rounded-lg px-4 py-2 text-center text-xl font-bold transition-colors"
    >Go back to dashboard</a
  >
</div>
