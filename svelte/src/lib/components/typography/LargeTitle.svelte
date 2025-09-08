<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<h1
  aria-label="Large Title"
  class="text-center text-3xl font-bold tracking-tight md:text-left md:text-5xl {styling} min-w-fit"
>
  {@render children?.()}
</h1>
