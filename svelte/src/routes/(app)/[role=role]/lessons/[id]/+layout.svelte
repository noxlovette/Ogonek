<script lang="ts">
  let { children, data } = $props();
</script>

{@render children()}

<svelte:head>
  <title>Занятие • {data.lesson.topic}</title>
</svelte:head>
