<script lang="ts">
  import { Header } from "$lib/components";

  let { children } = $props();
</script>

<div class="flex w-full flex-col items-center justify-center">
  {@render children()}
</div>
