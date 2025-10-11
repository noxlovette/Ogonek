<script lang="ts">
  import {
    Divider,
    Headline,
    Merger,
    Title1,
    UniButton,
    VStack,
  } from "$lib/components";
  import { Bell } from "@lucide/svelte";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

<Title1>Уведомления</Title1>
<VStack>
  <Headline>Телеграм</Headline>
  <Divider></Divider>
  <Merger>
    <UniButton
      variant="primary"
      href="https://t.me/fz_notif_bot"
      Icon={Bell}
      iconOnly={false}
      content="Включить уведомления"
    ></UniButton>
  </Merger>
</VStack>
