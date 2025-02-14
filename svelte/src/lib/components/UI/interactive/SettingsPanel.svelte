<script lang="ts">
  import { enhance } from "$app/forms";
  import { goto } from "$app/navigation";
  import {
    user,
    profile,
    notification,
    setProfile,
    setUser,
    initialProfile,
    initialUser,
    clearUser,
  } from "$lib/stores";
  import ButtonSubmit from "../buttons/ButtonSubmit.svelte";
  import ButtonRaw from "../buttons/ButtonRaw.svelte";
  import { H2 } from "$lib/components/typography";

  let isSubmitting = $state(false);
  let disabled = $state(true);
</script>

<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
  <form
    class="col-span-2 container"
    method="POST"
    use:enhance={({ formData, cancel }) => {
      if (!formData) cancel();

      isSubmitting = true;

      return async ({ result }) => {
        isSubmitting = false;

        // Destructure with nullish coalescing for safer fallbacks
        const { user = initialUser, profile = initialProfile } =
          result.data ?? {};

        if (result.type === "success" && result.data) {
          setUser(user);
          setProfile(profile);
          localStorage.setItem("user", JSON.stringify(user));
          localStorage.setItem("profile", JSON.stringify(profile));
          notification.set({ message: "Changes saved ✨", type: "success" });
          disabled = true;
        } else if (result.type === "failure") {
          notification.set({
            message: result.data?.message ?? "Yikes! Something's not right",
            type: "error",
          });
        }
      };
    }}
    action="?/update"
  >
    <div
      class="bg-milk-50 dark:bg-milk-900 grid grid-cols-1 gap-6 rounded-xl p-6 shadow md:grid-cols-2"
    >
      <!-- User Settings Section -->
      <div class="space-y-4">
        <h2 class="text-cacao-800 dark:text-milk-100 text-2xl font-bold">
          User Settings
        </h2>

        {#each ["email", "username", "name"] as field}
          <div class="space-y-2">
            <label
              for={field}
              class="text-cacao-700 dark:text-milk-100 block text-sm font-medium capitalize"
            >
              {field}
            </label>
            <input
              type={field === "email" ? "email" : "text"}
              id={field}
              {disabled}
              name={field}
              value={$user[field]}
              class="dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 disabled:text-milk-500 border-milk-200 dark:bg-milk-950 focus:ring-cacao-500 w-full rounded-lg border px-4 py-2
            transition duration-200 focus:ring-2 focus:outline-none
                   dark:focus:ring-2 dark:focus:outline-none"
            />
          </div>
        {/each}
      </div>

      <!-- Profile Settings Section -->
      <div class="space-y-4">
        <h2 class="text-cacao-800 dark:text-milk-100 text-2xl font-bold">
          Profile Settings
        </h2>
        {#each ["quizlet", "zoom"] as field}
          <div class="space-y-2">
            <label
              for={field}
              class="text-cacao-700 dark:text-milk-100 block text-sm font-medium capitalize"
            >
              {field} URL
            </label>
            <input
              type="url"
              {disabled}
              id={field}
              name={field}
              value={$profile[`${field}Url`]}
              class="dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 disabled:text-milk-500 border-milk-200 dark:bg-milk-950 focus:ring-cacao-500 w-full rounded-lg border px-4 py-2
            transition duration-200 focus:ring-2 focus:outline-none
                   dark:focus:ring-2 dark:focus:outline-none"
            />
          </div>
        {/each}
      </div>

      <!-- Update Button - Spans full width -->
      <div class="mt-6 flex flex-row space-x-2 md:col-span-2">
        <ButtonSubmit
          bind:isSubmitting
          buttonName="Save Changes"
          {disabled}
          styling="w-full md:w-auto md:float-right"
        />
        <button
          type="button"
          onclick={() => {
            disabled = !disabled;
          }}
          class="bg-milk-200 dark:bg-milk-800 dark:hover:bg-milk-700 hover:bg-milk-300 rounded-lg px-3 py-2 text-sm transition-colors md:px-4 md:text-base"
        >
          {disabled ? "Unlock" : "Lock"}
        </button>
      </div>
    </div>
  </form>

  <section
    class="bg-milk-50 dark:bg-milk-900 flex flex-col justify-between space-y-2 rounded-lg p-4 shadow"
  >
    <H2>Telegram</H2>
    <p class="align-top">
      Click the button below and type in "/start" to the bot if you want to
      receive notifications about new tasks
    </p>
    <a
      href="https://t.me/fz_notif_bot"
      class="flex max-w-fit rounded-lg bg-cyan-600 px-4 py-2 text-cyan-50 transition-colors hover:bg-cyan-700"
    >
      Receive notifications
    </a>
  </section>

  <form
    action="?/logout"
    method="POST"
    class="hidden md:flex"
    use:enhance={() => {
      isSubmitting = true;

      return async ({ result }) => {
        isSubmitting = false;
        if (result.type === "redirect") {
          clearUser();
          localStorage.removeItem("user");
          localStorage.removeItem("profile");
          notification.set({ message: "Bye!", type: "success" });
          goto(result.location);
        } else if (result.type === "failure") {
          notification.set({
            message: String(result.data?.message) || "Something's off",
            type: "error",
          });
        }
      };
    }}
  >
    <section
      class="bg-milk-50 dark:bg-milk-900 flex flex-col rounded-lg p-4 shadow"
    >
      <div class="flex-none">
        <H2>Log Out</H2>
      </div>

      <p class="my-2 flex-none">
        I didn't say it was gonna be easy, Neo. I just said it would be the
        truth.
      </p>

      <div class="mt-auto flex-none">
        <ButtonRaw styling="bg-red-600 hover:bg-red-700" buttonName="Log out" />
      </div>
    </section>
  </form>
</div>
