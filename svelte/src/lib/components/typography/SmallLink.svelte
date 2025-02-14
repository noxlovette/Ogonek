<script lang="ts">
  let { children, href = "/" } = $props();
</script>

<a {href} class="text-cacao-400 p-1 font-sans text-xs lg:text-sm">
  {@render children()}
</a>
