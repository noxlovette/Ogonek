<script lang="ts">
  let { href = "/", children } = $props();
</script>

<a
  {href}
  class="bg-cacao-600 text-cacao-50 hover:bg-cacao-700 focus:ring-cacao-500 rounded-lg px-4 py-2 transition-colors focus:ring focus:ring-offset-2 focus:outline-none disabled:opacity-50"
>
  {@render children()}
</a>
