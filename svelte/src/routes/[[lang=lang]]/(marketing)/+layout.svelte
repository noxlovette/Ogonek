<script>
  import Header from "$lib/components/UI/navigation/Header.svelte";

  let { children } = $props();
</script>

<Header></Header>
<div class="flex size-full flex-col p-12">
  {@render children()}
</div>
