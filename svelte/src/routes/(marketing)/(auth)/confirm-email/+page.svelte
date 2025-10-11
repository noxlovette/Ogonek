<script lang="ts">
  import { Callout, Caption1, UniButton } from "$lib/components";
  import Merger from "$lib/components/UI/toolbar/Merger.svelte";
  import { House } from "@lucide/svelte";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

{#if data.data?.tokenAbsent}
  <Caption1>Битая ссылка. Вернитесь на почту и попробуйте еще раз.</Caption1>
{:else if data.success}
  <Callout>Вы подтвердили почту</Callout>
{:else}
  <Callout>Что-то пошло не так. Возможно, вы уже подтверждали почту.</Callout>
{/if}
<Merger>
  <UniButton href="/" Icon={House} content="Вернуться на главную" />
</Merger>
