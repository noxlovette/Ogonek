<script lang="ts">
  interface Props {
    children?: import("svelte").Snippet;
  }

  let { children }: Props = $props();
</script>

<p class="text-tiny font-medium text-stone-500">
  {@render children?.()}
</p>
