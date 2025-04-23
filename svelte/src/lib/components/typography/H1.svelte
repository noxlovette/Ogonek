<script lang="ts">
  interface Props {
    children?: import("svelte").Snippet;
  }

  let { children }: Props = $props();
</script>

<h1
  class="text-center text-2xl font-bold tracking-tighter md:text-left md:text-4xl"
>
  {@render children?.()}
</h1>
