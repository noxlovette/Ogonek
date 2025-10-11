<script lang="ts">
  import { Title1, Caption1, UniButton } from "$lib/components";
  import Merger from "$lib/components/UI/toolbar/Merger.svelte";
  import { House } from "@lucide/svelte";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

{#if data.data?.tokenAbsent}
  <Caption1>Битая ссылка. Вернитесь на почту и попробуйте еще раз.</Caption1>
{:else if data.success}
  <Title1>Вы подтвердили почту</Title1>
{:else}
  <Title1>Что-то пошло не так. Возможно, вы уже подтверждали почту.</Title1>
{/if}
<Merger>
  <UniButton href="/" Icon={House} content="Вернуться на главную" />
</Merger>
