<script lang="ts">
  let { children, styling = "" } = $props();
</script>

<h5 class="text-base font-medium tracking-normal text-stone-700 {styling}">
  {@render children?.()}
</h5>
