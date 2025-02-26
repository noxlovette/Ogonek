<script lang="ts">
  import { enhance } from "$app/forms";
  import { notification } from "$lib/stores";
  import { goto } from "$app/navigation";
  import { Turnstile, ButtonSubmit } from "$lib/components";

  let loading = $state(false);
  let password = $state("");
  let confirmPassword = $state("");
  let passwordMatch = $state(true);
  let isSubmitting = $state(false);
</script>

<div
  class="dark:bg-milk-900 flex w-11/12 max-w-md flex-col items-center justify-center space-y-6 rounded-xl bg-white p-9 shadow-md"
>
  <div class="text-center">
    <h2 class="text-cacao-600 dark:text-milk-100 text-3xl font-bold">
      Create Account
    </h2>
    <p class="text-milk-600 mt-2 text-sm">
      Already have an account?
      <a
        href="/auth/login"
        class="dark:text-milk-100 text-cacao-500 hover:text-cacao-400 font-medium"
        >Sign in</a
      >
    </p>
  </div>

  <form
    method="post"
    class="w flex flex-col items-center justify-center space-y-4"
    use:enhance={() => {
      isSubmitting = true;

      return async ({ result }) => {
        isSubmitting = false;
        if (result.type === "redirect") {
          notification.set({ message: "Welcome on board", type: "success" });
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
    <div class="space-y-4">
      <div>
        <label for="name" class="text-milk-700 block text-sm font-medium"
          >Full Name</label
        >
        <input
          type="text"
          name="name"
          required
          class="dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 disabled:text-milk-500 border-milk-200 dark:bg-milk-950 focus:ring-cacao-500 w-full rounded-lg border px-4 py-2
            transition duration-200 focus:ring focus:outline-none
                   dark:focus:ring dark:focus:outline-none"
        />
      </div>

      <div>
        <label for="username" class="text-milk-700 block text-sm font-medium"
          >Username</label
        >
        <input
          type="text"
          name="username"
          required
          class="dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 disabled:text-milk-500 border-milk-200 dark:bg-milk-950 focus:ring-cacao-500 w-full rounded-lg border px-4 py-2
            transition duration-200 focus:ring focus:outline-none
                   dark:focus:ring dark:focus:outline-none"
        />
      </div>

      <div>
        <label for="role" class="text-milk-700 block text-sm font-medium"
          >Role</label
        >
        <select
          name="role"
          required
          class="dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 disabled:text-milk-500 border-milk-200 dark:bg-milk-950 focus:ring-cacao-500 w-full rounded-lg border px-4 py-2
            transition duration-200 focus:ring focus:outline-none
                   dark:focus:ring dark:focus:outline-none"
        >
          <option value="">Select a role</option>
          <option value="teacher">Teacher</option>
          <option value="student">Student</option>
        </select>
      </div>

      <div>
        <label for="email" class="text-milk-700 block text-sm font-medium"
          >Email</label
        >
        <input
          type="email"
          name="email"
          required
          class="dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 disabled:text-milk-500 border-milk-200 dark:bg-milk-950 focus:ring-cacao-500 w-full rounded-lg border px-4 py-2
            transition duration-200 focus:ring focus:outline-none
                   dark:focus:ring dark:focus:outline-none"
        />
      </div>

      <div>
        <label for="password" class="text-milk-700 block text-sm font-medium"
          >Password</label
        >
        <input
          type="password"
          name="password"
          bind:value={password}
          required
          class="dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 disabled:text-milk-500 border-milk-200 dark:bg-milk-950 focus:ring-cacao-500 w-full rounded-lg border px-4 py-2
            transition duration-200 focus:ring focus:outline-none
                   dark:focus:ring dark:focus:outline-none"
        />
      </div>

      <div>
        <label
          for="confirmPassword"
          class="text-milk-700 block text-sm font-medium"
          >Confirm Password</label
        >
        <input
          type="password"
          name="confirmPassword"
          bind:value={confirmPassword}
          required
          class="dark:focus:ring-milk-700 dark:focus:border-milk-800 dark:border-milk-800 disabled:text-milk-500 border-milk-200 dark:bg-milk-950 focus:ring-cacao-500 w-full rounded-lg border px-4 py-2
            transition duration-200 focus:ring focus:outline-none
                   dark:focus:ring dark:focus:outline-none"
        />
        {#if !passwordMatch}
          <p class="mt-1 text-sm text-red-600">Passwords don't match</p>
        {/if}
      </div>
    </div>
    <Turnstile />
    <ButtonSubmit bind:isSubmitting={loading} buttonName="Create Account"
    ></ButtonSubmit>
  </form>
</div>

<svelte:head>
  <title>Signup</title>
</svelte:head>
