<script lang="ts">
  import {
    LargeTitle,
    Input,
    UniButton,
    Toolbar,
    Divider,
    Merger,
    HStack,
    Title1,
    Title2,
    Headline,
    VStack,
    ThemeToggler,
  } from "$lib/components";
  import { enhance } from "$app/forms";
  import { page } from "$app/state";
  import { enhanceForm } from "$lib/utils";

  import { Check, LogOut, Bell, Ban, Pencil } from "@lucide/svelte";
  import {
    assigneeStore,
    notification,
    searchTerm,
    pageSize,
    currentPage,
    clearUser,
  } from "$lib/stores";
  import { goto } from "$app/navigation";
  import texts from "$lib/texts.js";

  let disabled = $state(true);

  const { data, form } = $props();
</script>

<svelte:head>
  <title>Настройки</title>
</svelte:head>

<form
  method="POST"
  use:enhance={enhanceForm({
    messages: {
      success: texts.crud.updated,
    },
    handlers: {
      success: async () => {
        disabled = true;
      },
    },
    shouldUpdate: true,
  })}
  class="flex flex-col gap-4"
  action="?/update"
>
  <Toolbar>
    <Divider />
    <VStack>
      <ThemeToggler />
      <Merger>
        {#if disabled}
          <UniButton
            Icon={Pencil}
            variant="prominent"
            onclick={() => {
              disabled = !disabled;
            }}
            type="button"
            content={texts.crud.edit}
          ></UniButton>
        {:else}
          <UniButton
            onclick={() => {
              disabled = !disabled;
            }}
            Icon={Ban}
            content={texts.crud.cancel}
          ></UniButton>
          <UniButton
            Icon={Check}
            type="submit"
            variant="prominent"
            disable={disabled}
            content={texts.crud.save}
            formaction="?/update"
          ></UniButton>
        {/if}
      </Merger>
    </VStack>
  </Toolbar>
  <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
    <HStack>
      <Title1>Настройки аккаунта</Title1>

      <HStack>
        <Input
          bind:disabled
          placeholder="Как вас зовут?"
          name="name"
          invalid={form?.name}
          invalidDescription="3+ знаков и никакой херни"
          labelName="Имя"
          value={data.user.name}
        />
        <Input
          bind:disabled
          type="email"
          placeholder="Email"
          name="email"
          invalid={form?.email}
          verified={data.user.verified}
          invalidDescription="Это не похоже на почту"
          labelName="Электронная почта"
          value={data.user.email}
        />
      </HStack>

      <!-- Teacher Settings -->
      {#if page.params.role === "t"}
        <Divider />
        <Title2>Настройки преподавателя</Title2>
        <Input
          bind:disabled
          placeholder="Your Telegram ID"
          name="telegramId"
          labelName="Телеграм ID"
          value={data.profile.telegramId}
        />
        <Input
          bind:disabled
          placeholder="The link for your lessons"
          name="videoCallUrl"
          invalid={form?.url}
          invalidDescription="Это не URL"
          labelName="Ссылка на видеозвонок"
          value={data.profile.videoCallUrl}
        />
      {/if}
    </HStack>

    <HStack>
      <HStack>
        <Title1>Настройки</Title1>
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
      </HStack>
      <!-- Logout Section -->
      <HStack>
        <VStack>
          <Headline>Log Out</Headline>
          <Divider></Divider>
          <form
            action="?/logout"
            method="POST"
            class="flex flex-col"
            use:enhance={enhanceForm({
              handlers: {
                redirect: async (result) => {
                  clearUser();
                  assigneeStore.set("");
                  pageSize.set(20);
                  currentPage.reset();
                  searchTerm.set("");
                  notification.set({ message: "Bye!", type: "success" });
                  goto(result.location);
                },
              },
            })}
          >
            <Merger>
              <UniButton
                variant="danger"
                type="submit"
                Icon={LogOut}
                iconOnly={false}
                content="Уйти"
              ></UniButton>
            </Merger>
          </form>
        </VStack>
      </HStack>
    </HStack>
  </div>
</form>
