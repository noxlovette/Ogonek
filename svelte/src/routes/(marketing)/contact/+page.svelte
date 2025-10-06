<script lang="ts">
  import { LargeTitle, UniButton } from "$lib/components";
  import { Mail, Send } from "@lucide/svelte";

  const contactInfo = {
    email: "contact@ogonek.app",
    telegram: "@noxlovette",
  };
</script>

<div class="mx-auto flex max-w-md flex-col items-center justify-center gap-4">
  <LargeTitle>Связаться с нами</LargeTitle>

  <div class="grid gap-4 md:grid-cols-2">
    <UniButton
      type="button"
      variant="primary"
      Icon={Mail}
      content="Почта"
      href={`mailto:${contactInfo.email}`}
    ></UniButton>

    <UniButton
      type="button"
      variant="primary"
      Icon={Send}
      content="Telegram"
      href="https://t.me/noxlovette"
    ></UniButton>
  </div>
</div>
