<script lang="ts">
  import { H1, H2, Input, UniButton } from "$lib/components";
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
  import HeaderEmbellish from "$lib/components/typography/HeaderEmbellish.svelte";

  let disabled = $state(true);

  function getFields(role: string) {
    // Start with the default first three elements
    const defaultFields = [
      { id: "name", label: "Name", type: "text", storeKey: "" },
      { id: "username", label: "Username", type: "text", storeKey: "" },
      { id: "email", label: "Email", type: "email", storeKey: "" },
    ];

    if (role === "t") {
      defaultFields.push({
        id: "telegramId",
        label: "Telegram ID",
        type: "text",
        storeKey: "telegramId",
      });

      defaultFields.push({
        id: "zoomUrl",
        label: "Zoom URL",
        type: "text",
        storeKey: "zoomUrl",
      });
    }

    return defaultFields;
  }

  const fields = getFields(page.params.role);
</script>

<svelte:head>
  <title>Settings</title>
</svelte:head>

<HeaderEmbellish>
  <H1>Settings</H1>
</HeaderEmbellish>
<div class="grid gap-6 md:grid-cols-2">
  <form
    class=""
    method="POST"
    use:enhance={enhanceForm({
      handlers: {
        success: async () => {
          notification.set({ message: "Changes saved ✨", type: "success" });
          disabled = true;
        },
      },
    })}
    action="?/update"
  >
    <div
      class="flex flex-col rounded-lg bg-inherit p-4 shadow-sm ring ring-stone-200 transition-all md:p-5 lg:p-6 dark:bg-stone-950 dark:ring-stone-800"
    >
      <div class="flex items-center justify-between">
        <H2>Account Settings</H2>

        <UniButton
          Icon={Key}
          variant="ghost"
          onclick={() => {
            disabled = !disabled;
          }}
          type="button"
        >
          {disabled ? "Edit" : "Editing..."}
        </UniButton>
      </div>

      <div class="grid grid-cols-1 gap-x-4 gap-y-3">
        {#each fields as field}
          <div>
            <Input
              type={field.type}
              placeholder={field.label}
              name={field.id}
              bind:disabled
              value={field.storeKey
                ? $profile[field.storeKey]
                : $user[field.id]}
            />
          </div>
        {/each}
      </div>

      <div class="mt-8 flex">
        <UniButton
          Icon={Check}
          type="submit"
          variant="primary"
          disable={disabled}>Save</UniButton
        >
      </div>
    </div>
  </form>

  <div class="grid gap-4">
    <section
      class="space-y-3 rounded-lg bg-inherit p-4 shadow-sm ring ring-stone-200 transition-all md:p-5 lg:p-6 dark:ring-stone-800"
    >
      <div class="flex items-center gap-3">
        <H2>Telegram Notifications</H2>
      </div>

      <p class="text-sm text-stone-700 dark:text-stone-300">
        Connect with our Telegram bot to receive instant notifications for new
        tasks, due dates, and other important updates.
      </p>

      <div class="flex">
        <UniButton
          variant="primary"
          Icon={Bell}
          href="https://t.me/fz_notif_bot"
        >
          Enable Notifications
        </UniButton>
      </div>
    </section>

    <!-- Logout Section -->
    <form
      action="?/logout"
      method="POST"
      class="flex h-full flex-col"
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
      <section
        class="flex flex-col justify-between space-y-3 rounded-lg bg-inherit p-4 shadow-sm ring ring-stone-200 transition-all md:p-5 lg:p-6 dark:ring-stone-800"
      >
        <div class="flex items-center gap-3">
          <H2>Account</H2>
        </div>

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
      </section>
    </form>
  </div>
</div>
