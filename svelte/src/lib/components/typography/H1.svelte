<script lang="ts">
  interface Props {
    children?: import("svelte").Snippet;
  }

  let { children }: Props = $props();
</script>

<h1 class="font-serif text-3xl font-bold tracking-tight">
  {@render children?.()}
</h1>
