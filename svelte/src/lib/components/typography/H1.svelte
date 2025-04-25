<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<h1
  class="text-center text-2xl font-bold tracking-tighter md:text-left md:text-4xl {styling}"
>
  {@render children?.()}
</h1>
