<script lang="ts">
  import { enhance } from "$app/forms";
  import { goto } from "$app/navigation";

  import { Input, Turnstile, UniButton } from "$lib/components";
  import {
    setProfile,
    setUser,
    initialUser,
    initialProfile,
    notification,
  } from "$lib/stores";
  import type { UserData } from "$lib/types";
  import { enhanceForm } from "$lib/utils";
  import { DoorOpen } from "lucide-svelte";

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
    class="w flex flex-col items-center justify-center space-y-4"
    use:enhance={enhanceForm({
      messages: {
        failure: "Something's off",
      },
      handlers: {
        success: async (result) => {
          if (result.data) {
            const { user = initialUser, profile = initialProfile } =
              result.data;
            setUser(user);
            setProfile(profile);
            localStorage.setItem("user", JSON.stringify(user));
            localStorage.setItem("profile", JSON.stringify(profile));
            notification.set({ message: "Welcome home", type: "success" });
            await goto(
              user.role === "teacher" ? "/t/dashboard" : "/s/dashboard",
            );
          }
        },
      },
    })}
  >
    <div class="">
      <Input name="username" placeholder="Username" value="" />
    </div>
    <div class="">
      <Input name="password" placeholder="Password" value="" type="password" />
    </div>
    <Turnstile />
    <UniButton Icon={DoorOpen} type="submit" variant="primary" fullWidth={true}
      >Login</UniButton
    >
  </form>
</div>

<svelte:head>
  <title>Login</title>
</svelte:head>
