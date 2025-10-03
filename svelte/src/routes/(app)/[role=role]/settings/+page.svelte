<script lang="ts">
  import {
    LargeTitle,
    Input,
    UniButton,
    Toolbar,
    Divider,
    Merger,
    HStack,
    Caption1,
    Title1,
    Title2,
    LanguageSelector,
    Headline,
    VStack,
    Title3,
    ThemeToggler,
  } from "$lib/components";
  import { enhance } from "$app/forms";
  import { page } from "$app/state";
  import { enhanceForm } from "$lib/utils";

  import { Check, LogOut, Bell, Ban, Pencil } from "@lucide/svelte";
  import { m } from "$lib/paraglide/messages";
  import {
    assigneeStore,
    notification,
    searchTerm,
    pageSize,
    currentPage,
    clearUser,
  } from "$lib/stores";
  import { goto } from "$app/navigation";

  let disabled = $state(true);

  const { data, form } = $props();
</script>

<svelte:head>
  <title>{m.settings()}</title>
</svelte:head>

<form
  method="POST"
  use:enhance={enhanceForm({
    messages: {
      success: m.changesSaved(),
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
    <LargeTitle>{m.settings()}</LargeTitle>

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
            content={disabled ? m.edit() : m.editing()}
          ></UniButton>
        {:else}
          <UniButton
            onclick={() => {
              disabled = !disabled;
            }}
            Icon={Ban}
            content={m.cancel()}
          ></UniButton>
          <UniButton
            Icon={Check}
            type="submit"
            variant="prominent"
            disable={disabled}
            content={m.save()}
            formaction="?/update"
          ></UniButton>
        {/if}
      </Merger>
    </VStack>
  </Toolbar>
  <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
    <HStack>
      <Title1>{m.noble_formal_zebra_sprout()}</Title1>

      <HStack>
        <Input
          bind:disabled
          placeholder="Name"
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
          invalidDescription="Это не похоже на почту"
          labelName="Электронная почта"
          value={data.user.email}
        />
      </HStack>

      <!-- Teacher Settings -->
      {#if page.params.role === "t"}
        <Divider />
        <Title2>{m.bad_fluffy_lionfish_expand()}</Title2>
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
        <Title1>{m.settings()}</Title1>
        <VStack>
          <Headline>
            {m.stale_quick_mantis_stab()}
          </Headline>
          <Divider></Divider>
          <UniButton
            variant="primary"
            href="https://t.me/fz_notif_bot"
            Icon={Bell}
            iconOnly={false}
            content={m.suave_teary_emu_expand()}
          ></UniButton>
        </VStack>
        <Caption1>
          {m.broad_clear_snake_peel()}
        </Caption1>
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
            <UniButton
              variant="danger"
              type="submit"
              Icon={LogOut}
              iconOnly={false}
              content={m.seemly_any_ostrich_believe()}
            ></UniButton>
          </form>
        </VStack>
        <Caption1>
          {m.odd_tough_shell_dust()}
        </Caption1>
      </HStack>
    </HStack>

    <HStack>
      <VStack>
        <Title3>{m.safe_lazy_cheetah_dial()}</Title3>
      </VStack>
      <LanguageSelector />
    </HStack>
  </div>
</form>
