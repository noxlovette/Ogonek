<script lang="ts">
  let { children } = $props();
</script>

<section
  class="ring-default transition-allflex flex-col gap-3 rounded-lg bg-inherit px-5 py-4 shadow-sm dark:bg-stone-950 "
>
  {@render children?.()}
</section>
