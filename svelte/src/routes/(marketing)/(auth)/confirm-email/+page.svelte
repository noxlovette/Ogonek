<script lang="ts">
  import { Callout, Caption1 } from "$lib/components";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

{#if data.data?.tokenAbsent}
  <Caption1>Битая ссылка. Вернитесь на почту и попробуйте еще раз</Caption1>
{:else if data.success}
  <Callout>Вы подтвердили почту</Callout>
{/if}
