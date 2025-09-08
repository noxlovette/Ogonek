<script lang="ts">
  import { LargeTitle, UniButton } from "$lib/components";
  import { m } from "$lib/paraglide/messages";
  import { Mail, Send } from "lucide-svelte";

  const contactInfo = {
    email: "danila.volkov@noxlovette.com",
    telegram: "@noxlovette",
  };
</script>

<div class="mx-auto flex max-w-md flex-col items-center justify-center gap-4">
  <LargeTitle>
    {m.mean_stock_dove_beam()}
  </LargeTitle>

  <div class="grid gap-4 md:grid-cols-2">
    <UniButton
      type="button"
      variant="primary"
      Icon={Mail}
      href={`mailto:${contactInfo.email}`}
    >
      Email</UniButton
    >

    <UniButton
      type="button"
      variant="primary"
      Icon={Send}
      href="https://t.me/noxlovette"
    >
      Telegram</UniButton
    >
  </div>

  <p class="text-center text-stone-600 dark:text-stone-400">
    {m.same_brief_stingray_assure()}
  </p>
</div>
