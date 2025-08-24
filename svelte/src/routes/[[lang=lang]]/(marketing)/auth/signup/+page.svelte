<script lang="ts">
  import { enhance } from "$app/forms";
  import { Turnstile, UniButton, Input, Label, H1 } from "$lib/components";
  import { DoorOpen } from "lucide-svelte";
  import { enhanceForm } from "$lib/utils";
  import { m } from "$lib/paraglide/messages";

  let password = $state("");
  let confirmPassword = $state("");
  let passwordMatch = $state(true);
</script>

<div class="flex max-w-md flex-col gap-4">
  <div class="flex flex-col items-center text-center">
    <H1>
      {m.bad_even_seahorse_rise()}
    </H1>
    <p class="mt-2 text-sm text-stone-600">
      {m.dark_candid_octopus_compose()}
      <a
        href="/auth/login"
        class="text-accent hover:text-accent font-medium dark:text-stone-100"
        >{m.logIn()}</a
      >
    </p>
  </div>

  <form
    method="post"
    class="ring-default items-center justify-center gap-4 rounded-lg p-6"
    use:enhance={enhanceForm({
      messages: {
        redirect: "Welcome on board",
        defaultError: "Signup Failed",
      },
      navigate: true,
    })}
  >
    <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
      <Input name="name" placeholder="Name" value=""></Input>
      <Input name="username" placeholder="Username" value=""></Input>

      <div class="flex flex-col space-y-1">
        <Label>Role</Label>
        <select
          name="role"
          required
          class="focus:border-accent focus:ring-accent h-full w-full rounded-2xl border border-stone-300 bg-white px-4 py-2 text-base text-stone-900 placeholder-stone-400 shadow-sm focus:shadow-md focus:ring-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-60 dark:border-stone-700 dark:bg-stone-950 dark:text-stone-100"
        >
          <option value="">Select a role</option>
          <option value="teacher">Teacher</option>
          <option value="student">Student</option>
        </select>
      </div>
      <Input name="email" placeholder="Email" type="email" value=""></Input>
      <Input
        name="password"
        placeholder="Password"
        type="password"
        bind:value={password}
      ></Input>
      <Input
        name="confirmPassword"
        placeholder="Again"
        type="password"
        bind:value={confirmPassword}
      ></Input>

      {#if !passwordMatch}
        <p class="mt-1 text-sm text-red-600">Passwords don't match</p>
      {/if}
    </div>
    <Turnstile />
    <UniButton Icon={DoorOpen} type="submit" variant="primary" fullWidth={true}
      >Create Account</UniButton
    >
  </form>
</div>

<svelte:head>
  <title>Signup</title>
</svelte:head>
