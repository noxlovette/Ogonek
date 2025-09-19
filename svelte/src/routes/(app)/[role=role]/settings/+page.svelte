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
  } from "$lib/components";
  import { enhance } from "$app/forms";
  import { page } from "$app/state";
  import { enhanceForm } from "$lib/utils";

  import { Check, LogOut, Key, Bell, Merge, Ban } from "lucide-svelte";
  import { m } from "$lib/paraglide/messages";
  import {
    user,
    assigneeStore,
    notification,
    searchTerm,
    pageSize,
    currentPage,
    clearUser,
    profile,
  } from "$lib/stores";
  import { goto } from "$app/navigation";
  import Headline from "$lib/components/typography/Headline.svelte";
  import VStack from "$lib/components/UI/VStack.svelte";
  import ThemeToggler from "$lib/components/UI/interactive/ThemeToggler.svelte";
  import Title3 from "$lib/components/typography/Title3.svelte";

  let disabled = $state(true);
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
            Icon={Key}
            variant="prominent"
            onclick={() => {
              disabled = !disabled;
            }}
            type="button"
          >
            {disabled ? m.edit() : m.editing()}
          </UniButton>
        {:else}
          <UniButton
            onclick={() => {
              disabled = !disabled;
            }}
            Icon={Ban}
          >
            {m.cancel()}
          </UniButton>
          <UniButton
            Icon={Check}
            type="submit"
            variant="prominent"
            disable={disabled}
            formaction="?/update">{m.save()}</UniButton
          >
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
          labelName="Имя"
          value={$user.name}
        />
        <Input
          bind:disabled
          type="email"
          placeholder="Email"
          name="email"
          labelName="Электронная почта"
          value={$user.email}
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
          value={$profile.telegramId}
        />
        <Input
          bind:disabled
          placeholder="The link for your lessons"
          name="videoCallUrl"
          labelName="Ссылка на видеозвонок"
          value={$profile.videoCallUrl}
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
          >
            {m.suave_teary_emu_expand()}
          </UniButton>
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
                  assigneeStore.reset();
                  pageSize.reset();
                  currentPage.reset();
                  searchTerm.reset();
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
            >
              {m.seemly_any_ostrich_believe()}
            </UniButton>
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
