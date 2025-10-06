<script lang="ts">
  import { enhanceForm } from "$lib/utils";
  import {
    BackButton,
    DateTimePicker,
    Divider,
    HStack,
    Input,
    Merger,
    SaveButton,
    SectionBg,
    Title1,
    VStack,
  } from "$lib/components";
  import { enhance } from "$app/forms";
  import Caption1 from "$lib/components/typography/Caption1.svelte";

  const { form } = $props();

  const now = new Date();
  const rounded = new Date(Math.round(now.getTime() / 3600000) * 3600000);
</script>

<svelte:head>
  <title>Новое событие • Календарь</title>
  <meta name="description" content="Добавить новое событие в Огонек" />
</svelte:head>
<form
  use:enhance={enhanceForm({
    messages: {
      redirect: "Событие добавлено",
    },
    navigate: true,
    shouldUpdate: true,
  })}
  method="POST"
  class="flex w-full flex-col gap-3 md:gap-3 lg:gap-4"
>
  <BackButton></BackButton>
  <VStack>
    <Title1>Создать Событие</Title1>
    <Divider />
    <Merger>
      <SaveButton />
    </Merger>
  </VStack>

  <HStack>
    <DateTimePicker {form} dtstartTime={rounded.toISOString()} />
  </HStack>
  <SectionBg>
    <HStack>
      <Input
        showLabel={false}
        invalid={form?.attendee}
        invalidDescription="Выберите ученика"
        placeholder="Выберите ученика"
        type="attendee"
        name="attendee"
        labelName="Ученик"
      />
    </HStack>
  </SectionBg>
  <SectionBg>
    <Caption1>
      Если вы настраивали ссылку на свою конференцию, она автоматически будет
      присвоена этому событию. Сделать это можно <a href="/t/settings">здесь</a
      >.
    </Caption1>
  </SectionBg>
</form>
