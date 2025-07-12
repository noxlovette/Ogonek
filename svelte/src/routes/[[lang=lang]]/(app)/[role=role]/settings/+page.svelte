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
  import { m } from "$lib/paraglide/messages";

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
  class="flex flex-col gap-3"
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
  action="?/update"
>
  <HeaderEmbellish>
    <div class="flex flex-col gap-3 md:flex-row md:gap-4">
      <H1>{m.settings()}</H1>
      <div class="flex gap-3 md:gap-4">
        <UniButton
          Icon={Key}
          variant="primary"
          onclick={() => {
            disabled = !disabled;
          }}
          type="button"
        >
          {disabled ? m.edit() : m.editing()}
        </UniButton>
        <UniButton
          Icon={Check}
          type="submit"
          variant="primary"
          disable={disabled}
          formaction="?/update">{m.save()}</UniButton
        >
      </div>
    </div>
  </HeaderEmbellish>

  <div class="grid gap-3 md:grid-cols-2">
    <Panel>
      <div>
        <H2>{m.mellow_mild_pig_boil()}</H2>
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
        <H2>{m.real_best_gibbon_dazzle()}</H2>
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
      <H2>{m.stale_quick_mantis_stab()}</H2>
    </div>

    <p class="text-sm text-stone-700 dark:text-stone-300">
      {m.broad_clear_snake_peel()}
    </p>

    <div class="flex">
      <UniButton variant="primary" Icon={Bell} href="https://t.me/fz_notif_bot">
        {m.suave_teary_emu_expand()}
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
    <p class="text-sm text-stone-700 dark:text-stone-300">
      {m.odd_tough_shell_dust()}
    </p>
    <div>
      <UniButton
        variant="danger"
        type="submit"
        Icon={LogOut}
        formaction="?/logout">{m.seemly_any_ostrich_believe()}</UniButton
      >
    </div>
  </Panel>
</form>
