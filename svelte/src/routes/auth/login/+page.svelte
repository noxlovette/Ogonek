<script lang="ts">
  import { enhance } from "$app/forms";
  import { goto } from "$app/navigation";
  import { ButtonSubmit } from "$lib/components/UI";
  import {Turnstile} from "$lib/components";
  import {
    setProfile,
    setUser,
    initialUser,
    initialProfile,
    notification,
  } from "$lib/stores";
  import type { UserData } from "$lib/types";

  let isSubmitting = $state(false);
</script>

<div
  class="dark:bg-milk-900 flex w-11/12 max-w-md flex-col items-center justify-center space-y-6 rounded-xl bg-white p-9 shadow-md"
>
  <div class="text-center">
    <h2 class="text-cacao-600 dark:text-milk-200 text-3xl font-bold">
      Welcome back
    </h2>
    <p class="text-milk-600 mt-2 text-sm">
      Don't have an account?
      <a
        href="/auth/signup"
        class="text-cacao-500 dark:text-milk-50 dark:hover:text-milk-200 hover:text-cacao-400 font-medium"
        >Sign up</a
      >
    </p>
  </div>
  <form
    method="POST"
    use:enhance={() => {
      isSubmitting = true;

      return async ({ result }) => {
        isSubmitting = false;

        if (result.type === "success" && result.data) {
          const { user = initialUser, profile = initialProfile } =
            result.data as unknown as UserData;
          setUser(user);
          setProfile(profile);
          localStorage.setItem("user", JSON.stringify(user));
          localStorage.setItem("profile", JSON.stringify(profile));
          notification.set({ message: "Welcome home", type: "success" });
          await goto(user.role === "teacher" ? "/t/dashboard" : "/s/dashboard");
        } else if (result.type === "failure") {
          notification.set({
            message: String(result.data?.message) || "Something's off",
            type: "error",
          });
        }
      };
    }}
    class="w flex flex-col items-center justify-center space-y-4"
  >
    <div class="">
      <div>
        <label for="username" class="text-milk-700 block text-sm font-medium">
          Username
        </label>
        <input
          id="username"
          name="username"
          type="text"
          class="dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 disabled:text-milk-500 border-milk-200 dark:bg-milk-950 focus:ring-cacao-500 w-full rounded-lg border px-4 py-2
            transition duration-200 focus:ring-2 focus:outline-none
                   dark:focus:ring-2 dark:focus:outline-none"
          required
          autocomplete="username"
        />
      </div>
    </div>
    <div class="">
      <div>
        <label for="password" class="text-milk-700 block text-sm font-medium">
          Password
        </label>
        <input
          id="password"
          name="password"
          type="password"
          class="dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 disabled:text-milk-500 border-milk-200 dark:bg-milk-950 focus:ring-cacao-500 w-full rounded-lg border px-4 py-2
            transition duration-200 focus:ring-2 focus:outline-none
                   dark:focus:ring-2 dark:focus:outline-none"
          required
          autocomplete="current-password"
        />
      </div>
    </div>

    <Turnstile />

    <ButtonSubmit bind:isSubmitting buttonName="Login" />
  </form>
</div>

<svelte:head>
  <title>Login</title>
</svelte:head>
