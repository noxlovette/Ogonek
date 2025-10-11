<script lang="ts">
  let { children } = $props();
</script>

<div class="bg-default ring-default grid rounded-xl">
  {@render children()}
</div>
