<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<h1
  class="{styling} text-center text-xl font-semibold tracking-tight text-balance md:text-left md:text-3xl"
>
  {@render children?.()}
</h1>
