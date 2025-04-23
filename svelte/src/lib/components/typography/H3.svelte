<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<h3
  class="text-2xl leading-6 font-medium tracking-normal text-balance sm:text-xl lg:text-2xl {styling}"
>
  {@render children()}
</h3>
