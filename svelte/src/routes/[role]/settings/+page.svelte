<script>
  import { H1, H2, H3, Input, UniButton } from "$lib/components";
  import { enhance } from "$app/forms";
  import { goto } from "$app/navigation";
  import { enhanceForm } from "$lib/utils";
  import {
    user,
    profile,
    notification,
    setProfile,
    setUser,
    initialProfile,
    initialUser,
    clearUser,
    assigneeStore,
    pageSize,
    searchTerm,
    currentPage,
  } from "$lib/stores";

  import { Check, LogOut, Key } from "lucide-svelte";

  let disabled = $state(true);

  const fieldGroups = [
    {
      title: "User Settings",
      fields: [
        { id: "name", label: "Name", type: "text", storeKey: "" },
        { id: "username", label: "Username", type: "text", storeKey: "" },
        { id: "email", label: "Email", type: "email", storeKey: "" },
      ],
    },
    {
      title: "Profile Settings",
      fields: [
        { id: "zoom", label: "Zoom URL", type: "text", storeKey: "zoomUrl" },
      ],
    },
  ];
</script>

<svelte:head>
  <title>Settings</title>
</svelte:head>

<H1>Settings</H1>

<div class="grid grid-cols-1 gap-6">
  <!-- Main Settings Form -->
  <form
    class=""
    method="POST"
    use:enhance={enhanceForm({
      messages: {
        failure: "Something's off",
      },
      handlers: {
        success: async (result) => {
          if (result.data) {
            const { user, profile } = result.data ?? {
              user: initialUser,
              profile: initialProfile,
            };
            setUser(user);
            setProfile(profile);
            localStorage.setItem("user", JSON.stringify(user));
            localStorage.setItem("profile", JSON.stringify(profile));
            notification.set({ message: "Changes saved ✨", type: "success" });
            disabled = true;
          }
        },
      },
    })}
    action="?/update"
  >
    <div
      class="rounded-xl border border-stone-100 bg-stone-50 p-6 shadow-sm transition-all dark:border-stone-800 dark:bg-stone-950"
    >
      <div
        class="mb-6 flex items-center justify-between border-b border-stone-200 pb-4 dark:border-stone-700"
      >
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

      <!-- Fields Grid -->
      <div class="grid grid-cols-1 gap-x-8 gap-y-6 md:grid-cols-2">
        {#each fieldGroups as group, i}
          <div>
            <H3>
              {group.title}
            </H3>
            <div class="mt-4 space-y-5">
              {#each group.fields as field}
                <Input
                  type={field.type}
                  placeholder={field.label}
                  name={field.id}
                  bind:disabled
                  value={field.storeKey
                    ? $profile[field.storeKey]
                    : $user[field.id]}
                />
              {/each}
            </div>
          </div>
        {/each}
      </div>

      <!-- Save Button - Spans full width -->
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

  <!-- Bottom thing with account tools -->
  <div class="grid grid-cols-1 gap-6">
    <!-- Telegram Section -->
    <section
      class="rounded-xl border border-stone-100 bg-stone-50 p-6 shadow-sm transition-all dark:border-stone-800 dark:bg-stone-900"
    >
      <div class="mb-3 flex items-center gap-3">
        <div class="rounded-full bg-cyan-100 p-2 dark:bg-cyan-900">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-5 w-5 text-cyan-600 dark:text-cyan-400"
            fill="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              d="M9.78 18.65l.28-4.23 7.68-6.92c.34-.31-.07-.46-.52-.19L7.74 13.3 3.64 12c-.88-.25-.89-.86.2-1.3l15.97-6.16c.73-.33 1.43.18 1.15 1.3l-2.72 12.81c-.19.91-.74 1.13-1.5.71L12.6 16.3l-1.99 1.93c-.23.23-.42.42-.83.42z"
            ></path>
          </svg>
        </div>
        <H2>Telegram Notifications</H2>
      </div>

      <p class="mb-4 text-sm text-stone-700 dark:text-stone-300">
        Connect with our Telegram bot to receive instant notifications for new
        tasks, due dates, and other important updates.
      </p>

      <a
        href="https://t.me/fz_notif_bot"
        class="flex w-full items-center justify-center gap-2 rounded-lg bg-cyan-600 px-4 py-2.5 font-medium text-white transition-colors hover:bg-cyan-700"
      >
        Connect to Telegram
      </a>
    </section>

    <!-- Logout Section -->
    <form
      action="?/logout"
      method="POST"
      use:enhance={enhanceForm({
        messages: {
          failure: "Something's off",
        },
        handlers: {
          redirect: async (result) => {
            clearUser();
            localStorage.removeItem("user");
            localStorage.removeItem("profile");
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
        class="rounded-xl border border-stone-100 bg-stone-50 p-6 shadow-sm transition-all dark:border-stone-800 dark:bg-stone-900"
      >
        <div class="mb-3 flex items-center gap-3">
          <div class="rounded-full bg-red-100 p-2 dark:bg-red-900/50">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-5 w-5 text-red-600 dark:text-red-400"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"
              />
            </svg>
          </div>
          <H2>Account</H2>
        </div>

        <p class="mb-4 text-sm text-stone-700 dark:text-stone-300">
          "I didn't say it was gonna be easy, Neo. I just said it would be the
          truth."
        </p>

        <UniButton
          variant="danger"
          type="submit"
          Icon={LogOut}
          formaction="?/logout">Log Out</UniButton
        >
      </section>
    </form>
  </div>
</div>
