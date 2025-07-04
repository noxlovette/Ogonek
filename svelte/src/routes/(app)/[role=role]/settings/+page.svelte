<script lang="ts">
  import {
    H1,
    H2,
    Input,
    Panel,
    UniButton,
    HeaderEmbellish,
    Caption,
  } from "$lib/components";
  import { enhance } from "$app/forms";
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { enhanceForm } from "$lib/utils";
  import {
    user,
    profile,
    notification,
    clearUser,
    assigneeStore,
    pageSize,
    searchTerm,
    currentPage,
  } from "$lib/stores";

  import { Check, LogOut, Key, Bell } from "lucide-svelte";

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
      label: "Video Call Url",
      type: "text",
      storeKey: "videoCallUrl",
    },
  ];
</script>

<svelte:head>
  <title>Settings</title>
</svelte:head>

<form
  method="POST"
  class="flex flex-col gap-3"
  use:enhance={enhanceForm({
    handlers: {
      success: async () => {
        notification.set({ message: "Changes saved", type: "success" });
        disabled = true;
      },
    },
  })}
  action="?/update"
>
  <HeaderEmbellish>
    <div class="flex flex-col gap-3 md:flex-row md:gap-4">
      <H1>Settings</H1>
      <div class="flex gap-3 md:gap-4">
        <UniButton
          Icon={Key}
          variant="primary"
          onclick={() => {
            disabled = !disabled;
          }}
          type="button"
        >
          {disabled ? "Edit" : "Editing..."}
        </UniButton>
        <UniButton
          Icon={Check}
          type="submit"
          variant="primary"
          disable={disabled}
          formaction="?/update">Save</UniButton
        >
      </div>
    </div>
  </HeaderEmbellish>

  <div class="grid gap-3 md:grid-cols-2">
    <Panel>
      <div>
        <H2>Account Settings</H2>
      </div>

      <div class="grid gap-3">
        {#each defaultFields as field, index (index)}
          <div>
            <Input
              type={field.type}
              placeholder={field.label}
              labelName={field.label}
              name={field.id}
              bind:disabled
              value={field.storeKey
                ? $profile[field.storeKey]
                : $user[field.id]}
            />
          </div>
        {/each}
      </div></Panel
    >
    {#if page.params.role === "t"}
      <Panel>
        <H2>Teacher Settings</H2>
        <div class="grid gap-3">
          {#each teacherFields as field, index (index)}
            <div>
              <Input
                type={field.type}
                placeholder={field.label}
                labelName={field.label}
                name={field.id}
                bind:disabled
                value={field.storeKey
                  ? $profile[field.storeKey]
                  : $user[field.id]}
              />
            </div>
          {/each}
          {#if $profile.zoomUrl === null}
            <Caption>Please define your Zoom URL</Caption>
          {/if}
        </div>
      </Panel>
    {/if}
  </div>

  <Panel>
    <div class="flex items-center gap-3">
      <H2>Telegram Notifications</H2>
    </div>

    <p class="text-sm text-stone-700 dark:text-stone-300">
      Connect with our Telegram bot to receive instant notifications for new
      tasks, due dates, and other important updates.
    </p>

    <div class="flex">
      <UniButton variant="primary" Icon={Bell} href="https://t.me/fz_notif_bot">
        Enable Notifications
      </UniButton>
    </div>
  </Panel>
</form>
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
  <Panel>
    <H2>Account</H2>

    <p class="text-sm text-stone-700 dark:text-stone-300">
      "I didn't say it was gonna be easy, Neo. I just said it would be the
      truth."
    </p>
    <div>
      <UniButton
        variant="danger"
        type="submit"
        Icon={LogOut}
        formaction="?/logout">Log Out</UniButton
      >
    </div>
  </Panel>
</form>
