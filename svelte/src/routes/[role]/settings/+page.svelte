<script>
  import { H1, H2 } from "$lib/components";
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
  import UniButton from "$lib/components/UI/UniButton.svelte";
  import { Check, LogOut } from "lucide-svelte";

  let disabled = $state(true);

  // Animation helper
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
        {
          id: "quizlet",
          label: "Quizlet URL",
          type: "url",
          storeKey: "quizletUrl",
        },
        { id: "zoom", label: "Zoom URL", type: "url", storeKey: "zoomUrl" },
      ],
    },
  ];
</script>

<svelte:head>
  <title>Settings</title>
</svelte:head>

<H1>Settings</H1>

<div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
  <!-- Main Settings Form -->
  <form
    class="col-span-1 md:col-span-2 lg:col-span-2"
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
      class="bg-milk-50 border-milk-100 dark:border-milk-800 dark:bg-milk-950 rounded-xl border p-6 shadow-md transition-all"
    >
      <div
        class="border-milk-200 dark:border-milk-700 mb-6 flex items-center justify-between border-b pb-4"
      >
        <h2 class="text-cacao-800 dark:text-milk-100 text-2xl font-bold">
          Account Settings
        </h2>

        <button
          type="button"
          onclick={() => {
            disabled = !disabled;
          }}
          class={`flex items-center gap-2 rounded-lg px-4 py-2 text-sm font-medium transition-all duration-200
            ${
              disabled
                ? "bg-cacao-100 text-cacao-700 hover:bg-cacao-200 dark:text-cacao-300 dark:bg-milk-800 dark:hover:bg-milk-700"
                : "bg-cacao-500 hover:bg-cacao-600 dark:bg-cacao-600 dark:hover:bg-cacao-700 text-white"
            }`}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-4 w-4"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d={disabled
                ? "M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"
                : "M8 11V7a4 4 0 118 0m-4 8v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2z"}
            />
          </svg>
          {disabled ? "Edit Profile" : "Editing..."}
        </button>
      </div>

      <!-- Fields Grid -->
      <div class="grid grid-cols-1 gap-x-8 gap-y-6 md:grid-cols-2">
        {#each fieldGroups as group, i}
          <div>
            <h3
              class="text-cacao-700 dark:text-milk-200 mb-4 text-sm font-medium tracking-wider uppercase"
            >
              {group.title}
            </h3>
            <div class="space-y-5">
              {#each group.fields as field}
                <div class="space-y-2">
                  <label
                    for={field.id}
                    class="text-milk-700 dark:text-milk-300 block text-sm font-medium"
                  >
                    {field.label}
                  </label>
                  <div class="relative">
                    <input
                      type={field.type}
                      id={field.id}
                      name={field.id}
                      {disabled}
                      value={field.storeKey
                        ? $profile[field.storeKey]
                        : $user[field.id]}
                      class="border-milk-300 focus:ring-cacao-500 disabled:bg-milk-100 text-milk-900 dark:text-milk-100 dark:border-milk-700
                      dark:bg-milk-800 dark:disabled:bg-milk-900
                      w-full rounded-lg bg-white px-4 py-2.5
                      transition duration-200 focus:border-transparent
                      focus:ring-2 disabled:cursor-not-allowed"
                    />
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/each}
      </div>

      <!-- Save Button - Spans full width -->
      <div class="mt-8 flex">
        <UniButton Icon={Check} type="submit" variant="primary">Save</UniButton>
      </div>
    </div>
  </form>

  <!-- Right sidebar with account tools -->
  <div class="grid grid-cols-1 gap-6">
    <!-- Telegram Section -->
    <section
      class="bg-milk-50 border-milk-100 dark:border-milk-800 dark:bg-milk-900 rounded-xl border p-6 shadow-md transition-all"
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

      <p class="text-milk-700 dark:text-milk-300 mb-4 text-sm">
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
        class="bg-milk-50 border-milk-100 dark:border-milk-800 dark:bg-milk-900 rounded-xl border p-6 shadow-md transition-all"
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

        <p class="text-milk-700 dark:text-milk-300 mb-4 text-sm">
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

<style>
  /* Add subtle hover animation to inputs */
  input:not([disabled]):hover {
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  }

  /* Override for dark mode */
  :global(.dark) input:not([disabled]):hover {
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
  }
</style>
