<script lang="ts">
  import { enhance } from "$app/forms";
  import { enhanceForm } from "$lib/utils";

  import texts from "$lib/texts";
  import {
    Caption1,
    Divider,
    HLine,
    InputNew,
    Merger,
    SaveButton,
    Title1,
    VStack,
  } from "$lib/components";
  import InputMerger from "$lib/components/UI/forms/InputMerger.svelte";

  const { data, form } = $props();

  let password: string | undefined = $state("");
  let confirmPassword: string | undefined = $state("");
</script>

<form
  method="POST"
  use:enhance={enhanceForm({
    messages: {
      success: texts.crud.updated,
    },
    shouldUpdate: true,
  })}
  class="flex flex-col gap-4"
  action="?/update"
>
  <VStack>
    <Title1>Основные</Title1>
    <Divider />
    <Merger>
      <SaveButton />
    </Merger>
  </VStack>
  <InputMerger>
    <InputNew
      placeholder="Как вас зовут?"
      name="name"
      invalid={form?.name}
      invalidDescription="3+ знаков и никакой херни"
      labelName="Ваше имя"
      value={data.user.name}
    />
    <HLine />
    <InputNew
      type="email"
      placeholder="Email"
      name="email"
      invalid={form?.email}
      verified={data.user.verified}
      invalidDescription="Это не похоже на почту"
      labelName="Электронная почта"
      value={data.user.email}
    />
    <HLine />
    <InputNew
      type="text"
      placeholder="Ник"
      name="username"
      invalid={form?.username || form?.conflict}
      invalidDescription={form?.username
        ? "3+ символов и без интересностей"
        : "Такой уже есть"}
      labelName="Ник"
      value={data.user.username}
    />
  </InputMerger>
  <InputMerger>
    <InputNew
      type="password"
      name="pass"
      placeholder="Новый пароль"
      labelName="Пароль"
      bind:value={password}
    />
    {#if password}
      <InputNew
        type="password"
        name="confirmPassword"
        placeholder="Еще раз"
        labelName="Повторите пароль"
        bind:value={confirmPassword}
      />
    {/if}
  </InputMerger>
  {#if password !== confirmPassword}
    <Caption1 override="text-red-400">Пароли не совпадают</Caption1>
  {/if}
</form>
