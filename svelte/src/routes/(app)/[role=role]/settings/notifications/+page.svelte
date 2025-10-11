<script lang="ts">
  import {
    Body,
    Caption1,
    Divider,
    Headline,
    Merger,
    Title1,
    UniButton,
    VStack,
  } from "$lib/components";
  import { Bell } from "@lucide/svelte";
  import type { PageProps } from "./$types";
  import InputMerger from "$lib/components/UI/forms/InputMerger.svelte";

  let { data }: PageProps = $props();
</script>

<Title1>Уведомления</Title1>
<InputMerger>
  <VStack styling="p-2.5 items-center justify-between">
    <div>
      <Body>Телеграм</Body>
      <Caption1
        >Позволит вам получать уведомления от учеников. Вы можете отписаться,
        удалив бот в телеграм</Caption1
      >
    </div>
    <Divider></Divider>
    <Merger>
      <UniButton
        variant="primary"
        href="https://t.me/fz_notif_bot"
        Icon={Bell}
        content="Включить уведомления"
      ></UniButton>
    </Merger>
  </VStack>
</InputMerger>
