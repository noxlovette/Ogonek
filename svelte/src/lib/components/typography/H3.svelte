<script lang="ts">
    let { children } = $props();
  </script>
  
  <h2
    class="text-md font-medium text-balance sm:text-lg lg:text-xl snug text-milk-500"
  >
    {@render children()}
  </h2>
  