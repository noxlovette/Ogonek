<script lang="ts">
  import { Caption1, Input, InputNew, Title1 } from "$lib/components";
  import InputMerger from "$lib/components/UI/forms/InputMerger.svelte";
  import HLine from "$lib/components/UI/layout/HLine.svelte";
  import type { PageProps } from "./$types";

  let { data, form }: PageProps = $props();
</script>

<Title1>Преподаватель</Title1>
<InputMerger>
  <InputNew
    placeholder="Your Telegram ID"
    name="telegramId"
    labelName="Телеграм ID"
    value={data.profile.telegramId}
    clarification="Позволит ученикам запрашивать у вас домашние задания."
  />
  <HLine />
  <InputNew
    placeholder="The link for your lessons"
    name="videoCallUrl"
    invalid={form?.url}
    invalidDescription="Это не URL"
    labelName="Ссылка на видеозвонок"
    clarification="Поддерживает только не меняющиеся ссылки. Будет добавлена в каждое событие в календаре, которое вы создадите с этого момента."
    value={data.profile.videoCallUrl}
  />
</InputMerger>
