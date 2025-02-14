<script lang="ts">
  let { href, children } = $props();
</script>

<a
  {href}
  class="dark:bg-milk-950 ring-milk-200 dark:ring-milk-900 shadow-milk-100 dark:shadow-milk-900
      hover:shadow-cacao-100
      dark:hover:shadow-cacao-500
      dark:hover:ring-cacao-500 hover:ring-cacao-400
      group
      relative flex
      h-[150px]
      flex-col
      overflow-hidden
      rounded-xl
      bg-white
      p-5
      ring-1
      shadow-lg transition-all duration-200 ease-out
      dark:shadow-none hover:dark:shadow-sm"
>
  {@render children()}

  <div
    class="dark:from-milk-950 pointer-events-none absolute right-0 bottom-0
    left-0 h-12 bg-gradient-to-t from-white
    to-transparent"
  ></div>
</a>
