<script lang="ts">
  let { children } = $props();
</script>

<h2
  class="text-lg leading-6 font-semibold tracking-tight text-balance sm:text-xl lg:text-2xl"
>
  {@render children()}
</h2>
