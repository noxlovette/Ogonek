<script lang="ts">
  import { enhance } from "$app/forms";
  import { notification } from "$lib/stores";
  import { goto } from "$app/navigation";
  import { Turnstile, UniButton } from "$lib/components";
  import { DoorOpen } from "lucide-svelte";
  import { enhanceForm } from "$lib/utils";

  let password = $state("");
  let confirmPassword = $state("");
  let passwordMatch = $state(true);
  let isSubmitting = $state(false);
</script>

<div
  class="flex w-11/12 max-w-md flex-col items-center justify-center space-y-6 rounded-xl bg-white p-9 shadow-md dark:bg-stone-900"
>
  <div class="text-center">
    <h2 class="text-cacao-600 text-3xl font-bold dark:text-stone-100">
      Create Account
    </h2>
    <p class="mt-2 text-sm text-stone-600">
      Already have an account?
      <a
        href="/auth/login"
        class="text-cacao-500 hover:text-cacao-400 font-medium dark:text-stone-100"
        >Sign in</a
      >
    </p>
  </div>

  <form
    method="post"
    class="w flex flex-col items-center justify-center space-y-4"
    use:enhance={enhanceForm({
      messages: {
        redirect: "Welcome on board",
        defaultError: "Signup Failed",
      },
      navigate: true,
    })}
  >
    <div class="space-y-4">
      <div>
        <label for="name" class="block text-sm font-medium text-stone-700"
          >Full Name</label
        >
        <input
          type="text"
          name="name"
          required
          class="focus:ring-cacao-500 w-full rounded-lg border border-stone-200 px-4 py-2 transition duration-200 focus:ring focus:outline-none disabled:text-stone-500
            dark:border-stone-800 dark:bg-stone-950 dark:focus:border-stone-800 dark:focus:ring
                   dark:focus:ring-stone-700 dark:focus:outline-none"
        />
      </div>

      <div>
        <label for="username" class="block text-sm font-medium text-stone-700"
          >Username</label
        >
        <input
          type="text"
          name="username"
          required
          class="focus:ring-cacao-500 w-full rounded-lg border border-stone-200 px-4 py-2 transition duration-200 focus:ring focus:outline-none disabled:text-stone-500
            dark:border-stone-800 dark:bg-stone-950 dark:focus:border-stone-800 dark:focus:ring
                   dark:focus:ring-stone-700 dark:focus:outline-none"
        />
      </div>

      <div>
        <label for="role" class="block text-sm font-medium text-stone-700"
          >Role</label
        >
        <select
          name="role"
          required
          class="focus:ring-cacao-500 w-full rounded-lg border border-stone-200 px-4 py-2 transition duration-200 focus:ring focus:outline-none disabled:text-stone-500
            dark:border-stone-800 dark:bg-stone-950 dark:focus:border-stone-800 dark:focus:ring
                   dark:focus:ring-stone-700 dark:focus:outline-none"
        >
          <option value="">Select a role</option>
          <option value="teacher">Teacher</option>
          <option value="student">Student</option>
        </select>
      </div>

      <div>
        <label for="email" class="block text-sm font-medium text-stone-700"
          >Email</label
        >
        <input
          type="email"
          name="email"
          required
          class="focus:ring-cacao-500 w-full rounded-lg border border-stone-200 px-4 py-2 transition duration-200 focus:ring focus:outline-none disabled:text-stone-500
            dark:border-stone-800 dark:bg-stone-950 dark:focus:border-stone-800 dark:focus:ring
                   dark:focus:ring-stone-700 dark:focus:outline-none"
        />
      </div>

      <div>
        <label for="password" class="block text-sm font-medium text-stone-700"
          >Password</label
        >
        <input
          type="password"
          name="password"
          bind:value={password}
          required
          class="focus:ring-cacao-500 w-full rounded-lg border border-stone-200 px-4 py-2 transition duration-200 focus:ring focus:outline-none disabled:text-stone-500
            dark:border-stone-800 dark:bg-stone-950 dark:focus:border-stone-800 dark:focus:ring
                   dark:focus:ring-stone-700 dark:focus:outline-none"
        />
      </div>

      <div>
        <label
          for="confirmPassword"
          class="block text-sm font-medium text-stone-700"
          >Confirm Password</label
        >
        <input
          type="password"
          name="confirmPassword"
          bind:value={confirmPassword}
          required
          class="focus:ring-cacao-500 w-full rounded-lg border border-stone-200 px-4 py-2 transition duration-200 focus:ring focus:outline-none disabled:text-stone-500
            dark:border-stone-800 dark:bg-stone-950 dark:focus:border-stone-800 dark:focus:ring
                   dark:focus:ring-stone-700 dark:focus:outline-none"
        />
        {#if !passwordMatch}
          <p class="mt-1 text-sm text-red-600">Passwords don't match</p>
        {/if}
      </div>
    </div>
    <Turnstile />
    <UniButton Icon={DoorOpen} type="submit" variant="primary"
      >Create Account</UniButton
    >
  </form>
</div>

<svelte:head>
  <title>Signup</title>
</svelte:head>
