<script lang="ts">
  import { enhance } from "$app/forms";
  import { Captcha, UniButton, Input, Grid } from "$lib/components";
  import { DoorOpen } from "lucide-svelte";
  import { enhanceForm } from "$lib/utils";
  import { m } from "$lib/paraglide/messages";

  let password = $state("");
  let confirmPassword = $state("");
  let passwordMatch = $state(true);
</script>

<form
  method="post"
  class="flex flex-col gap-4"
  use:enhance={enhanceForm({
    messages: {
      redirect: "Welcome on board",
      defaultError: "Signup Failed",
    },
    navigate: true,
  })}
>
  <Grid>
    <Input name="name" placeholder="Name" value=""></Input>
    <Input name="username" placeholder="Username" value=""></Input>

    <Input name="role" type="role" />
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
  </Grid>
  <Captcha />
  <UniButton Icon={DoorOpen} type="submit" variant="primary" iconOnly={false}
    >Create Account</UniButton
  >
</form>

<svelte:head>
  <title>{m.signUp()}</title>
</svelte:head>
