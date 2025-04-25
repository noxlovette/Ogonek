<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<h3
  class="text-xl leading-6 font-medium tracking-normal text-balance md:text-2xl {styling}"
>
  {@render children()}
</h3>
