<script lang="ts">
  let { children, data } = $props();
</script>

{@render children()}

<svelte:head>
  <title>Lesson • {data.lesson.topic}</title>
</svelte:head>
