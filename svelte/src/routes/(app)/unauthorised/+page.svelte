<script lang="ts">
  import { ArrowLeft } from "@lucide/svelte";
  import { HStack, LargeTitle, Merger, UniButton } from "$lib/components";
</script>

<div class="flex size-full flex-col items-center justify-center">
  <HStack>
    <LargeTitle>Вам здесь не рады</LargeTitle>

    <Merger>
      <UniButton
        Icon={ArrowLeft}
        content="Восвояси"
        iconOnly={false}
        onclick={() => window.history.back()}
      ></UniButton>
    </Merger>
  </HStack>
</div>
<svelte:head>
  <title>Доступ запрещен</title>
</svelte:head>
