<script lang="ts">
  import { enhance } from "$app/forms";
  import { goto } from "$app/navigation";
  import { Input, Captcha, UniButton, Grid, Merger } from "$lib/components";
  import { m } from "$lib/paraglide/messages";
  import { initialUser, notification } from "$lib/stores";
  import { enhanceForm } from "$lib/utils";
  import { DoorOpen } from "lucide-svelte";
</script>

<form
  method="POST"
  class="flex max-w-md flex-col gap-2 md:gap-3 lg:gap-4"
  use:enhance={enhanceForm({
    handlers: {
      success: async (result) => {
        if (result.data) {
          const { user = initialUser } = result.data;

          notification.set({ message: "Welcome home", type: "success" });
          await goto(user.role === "teacher" ? "/t/dashboard" : "/s/dashboard");
        }
      },
    },
  })}
>
  <Grid>
    <Input
      required={true}
      showLabel={false}
      name="username"
      placeholder="Username"
      value=""
    />
    <Input
      required={true}
      name="password"
      showLabel={false}
      placeholder="Password"
      value=""
      type="password"
    />
  </Grid>
  <Captcha />
  <Merger>
    <UniButton Icon={DoorOpen} type="submit" variant="primary" iconOnly={false}
      >{m.logIn()}</UniButton
    >
  </Merger>
</form>

<svelte:head>
  <title>{m.logIn()}</title>
</svelte:head>
