<script lang="ts">
  import { enhance } from "$app/forms";
  import { goto } from "$app/navigation";
  import { H1, Input, Turnstile, UniButton } from "$lib/components";
  import { m } from "$lib/paraglide/messages";
  import { initialUser, notification } from "$lib/stores";
  import { enhanceForm } from "$lib/utils";
  import { DoorOpen } from "lucide-svelte";
</script>

<div class="flex max-w-md flex-col gap-4">
  <div class="flex flex-col items-center text-center">
    <H1>{m.zany_few_goose_mop()}</H1>
    <p class="mt-2 text-sm text-stone-600">
      {m.petty_neat_emu_endure()}
      <a
        href="/auth/signup"
        class="text-cacao-500 hover:text-cacao-400 font-medium dark:text-stone-50 dark:hover:text-stone-200"
        >{m.signUp()}</a
      >
    </p>
  </div>
  <form
    method="POST"
    class="ring-default flex flex-col gap-4 rounded-lg p-6"
    use:enhance={enhanceForm({
      handlers: {
        success: async (result) => {
          if (result.data) {
            const { user = initialUser } = result.data;

            notification.set({ message: "Welcome home", type: "success" });
            await goto(
              user.role === "teacher" ? "/t/dashboard" : "/s/dashboard",
            );
          }
        },
      },
    })}
  >
    <Input name="username" placeholder="Username" value="" />
    <Input name="password" placeholder="Password" value="" type="password" />
    <Turnstile />
    <UniButton Icon={DoorOpen} type="submit" variant="primary"
      >{m.logIn()}</UniButton
    >
  </form>
</div>

<svelte:head>
  <title>{m.logIn()}</title>
</svelte:head>
