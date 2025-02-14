<script lang="ts">
  let { children, href = "/" } = $props();
</script>

<a
  {href}
  target="_blank"
  rel="noopener noreferrer"
  class="inline-flex transition-all md:px-1 md:py-2 md:hover:translate-x-1 lg:px-2 lg:py-4"
>
  {@render children()}
</a>
