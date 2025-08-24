<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<h1
  class="text-center text-4xl font-bold tracking-tight md:text-left md:text-5xl {styling}"
>
  {@render children?.()}
</h1>
