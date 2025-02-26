<script lang="ts">
  let { href = "/", children } = $props();
</script>

<a
  {href}
  class="dark:hover:bg-milk-900 ring-milk-200 dark:ring-milk-800 dark:hover:ring-milk-950 bg-cacao-600 text-cacao-50 dark:bg-milk-800 dark:text-cacao-100 hover:bg-cacao-700 focus:ring-cacao-500 flex items-center justify-center rounded-lg px-3 py-2 text-sm ring transition-all focus:ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50 md:px-4 md:text-base"
>
  {@render children()}
</a>
