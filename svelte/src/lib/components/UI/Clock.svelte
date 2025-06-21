<script lang="ts">
  import { onMount } from "svelte";
  let currentTime = new Date();
  let formattedTime = $state("");

  onMount(() => {
    currentTime = new Date();
    formattedTime = new Intl.DateTimeFormat("en-US", {
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
      hour12: false,
    }).format(currentTime);
  });

  $effect(() => {
    setInterval(() => {
      currentTime = new Date();
      formattedTime = new Intl.DateTimeFormat("en-US", {
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
        hour12: false,
      }).format(currentTime);
    }, 1000);
  });
</script>

<div
  class="hidden text-stone-600 oldstyle-nums tabular-nums lg:text-xl xl:flex"
>
  {formattedTime}
</div>
