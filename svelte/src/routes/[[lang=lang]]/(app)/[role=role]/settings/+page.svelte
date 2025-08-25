<script lang="ts">
  import {
    LargeTitle,
    Input,
    UniButton,
    Toolbar,
    Divider,
    Merger,
    HStack,
    Title3,
    Caption1,
    Title1,
    Title2,
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

  let disabled = $state(true);

  const defaultFields = [
    { id: "name", label: "Name", type: "text", storeKey: "" },
    { id: "username", label: "Username", type: "text", storeKey: "" },
    { id: "email", label: "Email", type: "email", storeKey: "" },
  ];

  const teacherFields = [
    {
      id: "telegramId",
      label: "Telegram ID",
      type: "text",
      storeKey: "telegramId",
    },
    {
      id: "videoCallUrl",
      label: m.warm_fit_antelope_bump(),
      type: "text",
      storeKey: "videoCallUrl",
    },
  ];
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
      <Title1>Profile Settings</Title1>

      <HStack>
        <Input
          bind:disabled
          placeholder="Name"
          name="name"
          value={$user.name}
        />
        <Input
          bind:disabled
          type="email"
          placeholder="Email"
          name="email"
          value={$user.email}
        />
      </HStack>

      <!-- Teacher Settings -->
      {#if page.params.role === "t"}
        <Divider />
        <Title2>Teacher Settings</Title2>
        <Input
          bind:disabled
          placeholder="Your Telegram ID"
          name="telegramId"
          value={$profile.telegramId}
        />
        <Input
          bind:disabled
          placeholder="The link for your lessons"
          name="videoCallUrl"
          value={$profile.videoCallUrl}
        />
      {/if}
    </HStack>

    <HStack>
      <HStack>
        <Title1>Notifications</Title1>
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
  </div>
</form>
