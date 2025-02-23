<script lang="ts">
  let { target = "", children } = $props();
</script>

<label class="text-sm font-semibold capitalize opacity-60" for={target}
  >{@render children()}</label
>
