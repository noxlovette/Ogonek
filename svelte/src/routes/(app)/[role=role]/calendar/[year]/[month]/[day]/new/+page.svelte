<script lang="ts">
  import {
    parseRRuleToText,
    getLocaleFromCookie,
    isVideoCallUrl,
    enhanceForm,
  } from "$lib/utils";
  import { ChevronLeft, MapPin, Share, Video } from "lucide-svelte";
  import {
    BackButton,
    Body,
    Callout,
    CancelButton,
    Caption1,
    DateTimePicker,
    DeleteButton,
    Divider,
    Headline,
    HStack,
    Input,
    Merger,
    SaveButton,
    SectionBg,
    Title1,
    VStack,
  } from "$lib/components";
  import { enhance } from "$app/forms";
  import { goto } from "$app/navigation";
  import { date } from "zod";

  const { form, data } = $props();
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
    <DateTimePicker dtstart={data.date.toISOString()} />
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
      />
    </HStack>
  </SectionBg>
</form>
