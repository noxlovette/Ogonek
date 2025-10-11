<script lang="ts">
  import { Input, Title1 } from "$lib/components";
  import type { PageProps } from "./$types";

  let { data, form }: PageProps = $props();
</script>

<Title1>Настройки преподавателя</Title1>
<Input
  placeholder="Your Telegram ID"
  name="telegramId"
  labelName="Телеграм ID"
  value={data.profile.telegramId}
/>
<Input
  placeholder="The link for your lessons"
  name="videoCallUrl"
  invalid={form?.url}
  invalidDescription="Это не URL"
  labelName="Ссылка на видеозвонок"
  value={data.profile.videoCallUrl}
/>
